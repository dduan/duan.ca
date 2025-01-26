# GRPC Status With Details in Swift
2025-01-25T19:53:30-08:00
tag: Swift, GRPC, Protobuf, Go

## Introduction

In GRPC, one could define an RPC that, in addition to taking a request message,
and a response message, it also defines a custom message to represent errors:


```proto
message SignUpWithEmailRequest {
  string email = 1;
  string password = 2;
  string referral_code = 3;
}

message SignUpWithEmailResponse {
  AccessTokenDTO token = 1;
}

message SignUpWithEmailErrorResponse {
  enum Kind {
    KIND_UNKNOWN = 0;
    KIND_EMAIL_ALREADY_REGISTERED = 1;
    KIND_INVALID_PASSWORD = 2;
    KIND_INVALID_EMAIL = 3;
    KIND_INVALID_CODE = 4;
  }

  Kind kind = 1;
  repeated string reasons = 2;
}

service AuthenticationService {
  rpc SignUpWithEmail(SignUpWithEmailRequest) returns (SignUpWithEmailResponse) {}
}
```

... in this example, `SignUpWithEmailErrorResponse` is not directly referenced
in by `AuthenticationService`. But a server can use it as GRPC status with
details. In Go the code might look like this:

```go
import (
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

// ...

_, err = queries.GetUserByEmail(ctx, email)
if err == nil {
    response := &SignUpWithEmailErrorResponse{
        Kind:    SignUpWithEmailErrorResponse_KIND_EMAIL_ALREADY_REGISTERED,
        Reasons: []string{},
    }

    st := status.New(codes.AlreadyExists, "Email already registered")
    stWithDetails, err := st.WithDetails(response)
    if err != nil {
        return nil, err
    }

    return nil, stWithDetails.Err()
}
```

This is all very type-safe, very demure, until you realize that in grpc-swift
1.X there's no API to retrieve this "status with detail". When the information
is transimitted over the wire, you will have to dig it out manually. In this
post, I'll document how I did this with a client-side interceptor.

## The Swift interceptor

In Swift, when you make the RPC request, you'll get a standard error code and
error message if the server returns an error with the code shown earlier:

```swift
// Use the generated client code to make the gRPC request
var request = SignUpWithEmailRequest()
request.email = // ...
request.password = // ...
request.referralCode = // ...
do {
    let response = try await client.signUpWithEmail(request)
} catch {
    guard let error = error as? GRPCStatus else {
        print("Error: \(error)")
        return
    }
    print(error.code) // AlreadyExists
    print(error.message) // "Email already registered"
}
```

But, you, being a responsible client developer/tech lead/CTO, want to use the
type-safe enum from the protobuf definition so that you can display the error in
the right context, or perhaps localize it properly.

Here's the big picture: there maybe many such custom RPC error types. Our
solution should be universal, and flexible to handle each of them. Enter
interceptors! I mean, chances are, you know about them because you are working
with gRPC. Let's write one to get our type-safe status details. Starting with
a custom receive method, for the ".end" part of the response:

```swift
final class GRPCErrorDetailsInterceptor<Request, Response>:
  ClientInterceptor<Request, Response>, @unchecked Sendable
{
  override func receive(
    _ part: GRPCClientResponsePart<Response>,
    context: ClientInterceptorContext<Request, Response>
  ) {
    switch part {
    case .end(var status, let headers):
      // extract the error details, and forward it.
    default:
      context.receive(part)
    }
  }
}
```

... the "end" part contains the error status, as well as some trailing metadata.
The metadat includes our status details under the key `grpc-status-details-bin`.
It's base64 encoded, so we'll need to decode it...


```swift
    switch part {
    case .end(var status, let headers):
        guard
            // grab the status details
            let statusDetails = headers["grpc-status-details-bin"].first,
            // decode to data
            let data = Data(base64Encoded: statusDetails),
        // ...
    default:
      context.receive(part)
    }
```

At this point, with some experience with GRPC in Swift, you might think it's
time to instantiate your custom error type with `.init(seralizedData:)`. But
there'd be 2 problems:

1. You don't want each custom types from protobuf to make an appearance in an
   interceptor.
2. This data would not be in the right shape, despite what the metadata key
   says.

In fact, the data is of the well-known type `Google_Rpc_Status`. And our stutus
details, well, one its `.details` element. So:

```swift
    switch part {
    case .end(var status, let headers):
        guard
            let statusDetails = headers["grpc-status-details-bin"].first,
            let data = Data(base64Encoded: statusDetails),
            // the data, despite being under "grpc-status-details-bin", is
            // indeed not the status detail, but the statu itself:
            let googleStatus = try? Google_Rpc_Status(serializedData: data)
            // and the `details` field contains the actual status detail:
            let details = googleStatus.details.first,
        else {
            context.receive(part)
            break
        }
        // ...
    default:
      context.receive(part)
    }
```

... `details` is of type `Google_Protobuf_Any`. It is indeed a payload with the
content for `SignUpWithEmailErrorResponse` as defined in the Protobuf. One
question remains: how do we pass it from our intereceptor to the RPC call site?

Look at the call site from earlier: we have 2 code paths. If the call succeeds,
we get a `SignUpWithEmailResponse`. If it fails, we get a `GRPCStatus` as the
thrown error. Lucky for us, `GRPCStatus` has an unused field, `cause`. In my
version of `grpc-swift`, the field has the following docstring:

```swift
/// The cause of an error (not 'ok') status. This value is never transmitted
/// over the wire and is **not** included in equality checks.
public var cause: Error? { ... }
```

It seems like a perfect vessel for our status details!

```swift
    switch part {
    case .end(var status, let headers):
        guard
            let statusDetails = headers["grpc-status-details-bin"].first,
            let data = Data(base64Encoded: statusDetails),
            let googleStatus = try? Google_Rpc_Status(serializedData: data)
            let details = googleStatus.details.first,
        else {
            context.receive(part)
            break
        }
        // isn't it convenient that we declared `status` as a `var` ealier 😉?
        status.cause = details
        // forward to the caller, yay!
        context.receive(.end(status, headers))
    default:
      context.receive(part)
    }
```

Now our client will get the details of type `Google_Protobuf_Any` from the
`.cause` field of the thrown error. The client can proceed to decode it using
the protobuf-generated specific type with its built-in `.init(decodingAny:)`
initializer:


```swift
// Use the generated client code to make the gRPC request
var request = SignUpWithEmailRequest()
request.email = // ...
request.password = // ...
request.referralCode = // ...
do {
    let response = try await client.signUpWithEmail(request)
} catch {
    guard let error = error as? GRPCStatus else {
        print("Error: \(error)")
        return
    }

    // let's be type-safe, finally!
    guard
        let details = error.cause as? Google_Protobuf_Any,
        let signUpError = try? SignUpWithEmailErrorResponse(decodingAny: details)
    else {
        print("Error: \(error)")
        return
    }

    // 🎉
    switch signUpError.kind {
    // ...
    }
}
```

## Conclusion

I find this to be clean, targeted solution. Knowing the error detail's
transmission format is key to making this work. The fact that we also got
a clean architecture from exploiting an unused field is also very cool.
