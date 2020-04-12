+++
title = "Smart Color Invert And Your iOS Apps"
date = 2017-12-20T13:17:44-08:00
[taxonomies]
tags = ["iOS", "Accessibility", "Interface Builder"]
+++

1. [Introduction](#introduction)
2. [The Basics](#the-basics)
3. [Compatibility with older iOS versions](#compatibility-with-older-ios-versions)
4. [How to screenshot with inverted color settings](#how-to-screenshot-with-inverted-color-settings)
5. [Conclusion](#conclusion)

## Introduction

Apple sneakily shipped a "dark mode" in iOS 11, native apps looks _gorgeous_ in this mode (especially on
iPhone X with it's OLED display):

<img src="/assets/2017/12/native-ios-apps-with-smart-invert.png" srcset="/assets/2017/12/native-ios-apps-with-smart-invert.png 2x" alt="iOS 11 apps in smart invert mode"/>

To see it for yourself, go to _Settings -> General -> Accessibility Display Accommodations -> Invert Colors_
and toggle the switch to on (I personally set triple tap of the power buttton to toggle this on/off):

<img src="/assets/2017/12/invert-color-settings.png" srcset="/assets/2017/12/invert-color-settings.png" alt="turn on smart invert mode in Settings"/>

Note that in Apple's first-party apps, not every view's color is inverted (hence, "smart"). For apps that we
built, that would not be the case automatically. Instead, we get the full color inversion under this setting:

<img src="/assets/2017/12/third-party-app-no-change.png" srcset="/assets/2017/12/third-party-app-no-change.png 2x" alt="default third party app before and after smart color"/>

In this article, we'll see how to best adapt our apps so they look just as good as first-party iOS apps under
the smart invert setting.

## The Basics

_TL;DR: use [accessibilityIgnoresInvertColors][0] on your `UIView`s to prevent their color to be
inverted._

iOS 11 introduced `accessibilityIgnoresInvertColors`, a property on `UIView`. From the [offical
documentation][0]:

> If inverting the colors would have a negative impact on your view's content, set this property to true to
> prevent it from inverting its colors. Setting the property to true prevents the system from inverting the
> colors of the view and all of its subviews.

So, for views containing photos or videos with real-world content, we probably want to set it to `true`. In
our sample app's source code:

![setting accessibilityIgnoresInvertColors to true in code](/assets/2017/12/setting-accessibilityIgnoresInvertColors-to-true-in-code.png)

This gives us the desired effect for that view 🎉:

<img src="/assets/2017/12/smart-invert-in-code-result.png" srcset="/assets/2017/12/smart-invert-in-code-result.png 2x" alt="result of supporting invert color in code"/>

The same can be done in Interface Builder. Let's fix the other view by setting a runtime attribute:

![setting accessibilityIgnoresInvertColors to true in Interface Builder](/assets/2017/12/setting-accessibilityIgnoresInvertColors-to-true-in-interface-builder.png)

Result:

<img src="/assets/2017/12/smart-invert-in-interface-builder-result.png" srcset="/assets/2017/12/smart-invert-in-interface-builder-result.png 2x" alt="result of supporting invert color in interface builder"/>

Tada!

## Compatibility with older iOS versions

The story gets slightly complicated when your app need to run on older iOS versions. For one, our code won't
compile:

![compile error on older iOS versions](/assets/2017/12/compile-error-on-older-oses.png)

But that's nothing new to iOS developers, really. We can fix that with a `#available`:

![fix compile error on older iOS versions](/assets/2017/12/fix-compile-error.png)

When you run the app in older iOS devices or simulators, Xcode will complain about our runtime attribute as
well:

![Xcode complains about runtime attribute on older iOS](/assets/2017/12/xcode-complain-about-runtime-attirbute.png)

If that bothers you, I recommend writing your own wrapper for this property as an extension to `UIView`. To
put a cherry on top, make it `@IBInspectable`!


```swift
extension UIView {
    /// Whether or not to set accessibilityIgnoresInvertColors to true for iOS 11.
    /// For older OS, this value is false and setting it has no effect.
    @IBInspectable
    public var ignoreColorInversion: Bool {
        get {
            if #available(iOS 11, *) {
                return self.accessibilityIgnoresInvertColors
            }

            return false
        }

        set {
            if #available(iOS 11, *) {
                self.accessibilityIgnoresInvertColors = newValue
            }
        }
    }
}
```

Having this snippet in the project, we set it in interface builder to any view with a couple of clicks:

![set custom invert color properties in Interface Builder](/assets/2017/12/set-custom-invert-property-in-interface-builder.png)

## How to screenshot with inverted color settings

Here's a bonus hint: screenshots taken on device won't have their color inverted even if you have the setting
set to "on"! Go ahead, try it :).

To show off our handy work for supporting smart invert color, we'll need
some other means to take screenshots. All screenshots in this article is taken using QuickTime on a Mac. So
plug in your iOS device, open QuickTime Player, in menu select _File -> New Movie Recording_ and select your
device from the drop-down menu by clicking the arrow next to the record button:


![Select iOS device for screenshots in QuickTime
Player](/assets/2017/12/select-ios-device-in-quicktime-player.png)

Now you can screen shot the QuickTime Player window the normal way.

## Conclusion

You can [download the sample project][1] and play with it.

Smart invert is awesome and I personally wish all 3rd-party apps will update their apps for it as time goes
on. If your favorite app (including your own!) doesn't support it properly, please consider informing the
developer about [accessibilityIgnoresInvertColors][0], or just send this article their way!

[0]: https://developer.apple.com/documentation/uikit/uiview/2865843-accessibilityignoresinvertcolors
[1]: /assets/2017/12/SmartInvert.zip
