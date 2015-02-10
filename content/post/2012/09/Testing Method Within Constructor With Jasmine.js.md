+++
date = "2012-09-30T00:00:00-06:00"
title = "Testing Method Within Constructor With Jasmine.js"
tags = [ "jasmine.js", "javascript", "testing" ]
slug = "Testing-Method-Within-Constructor-With-Jasminejs"
+++

How do you test whether a method is called witin the constructor of an object
with Jasmine.js?

Turns out, you need to `spyOn()` the "raw" reference of the "method", which
really just a function on the `prototype` of your "class" object. An example
will make it clear:

    :::JavaScript
    // is 'load()' getting called during construction?
    describe("MyObject", function() {
      it("should call load() during construction", function() {
        spyOn(MyObject.prototype, 'load');
        new MyObject();
        expect(MyObject.prototype.load).toHaveBeenCalled();
      });
    }
