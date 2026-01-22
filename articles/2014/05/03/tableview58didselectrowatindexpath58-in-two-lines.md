# tableView:didSelectRowAtIndexPath: In Two Lines
2014-05-03T13:42:00-06:00
tag: Cocoa, Objective-C, Cocoa Touch, iOS

You have a `UITableViewController` with a couple of static cells, you want to
invoke some code for each cell in the delegate. Here's a quick way to do it:

        - (void)tableView:(UITableView *)tableView didSelectRowAtIndexPath:(NSIndexPath *)indexPath
        {
            
            SEL action = (SEL[]){@selector(method1), @selector(method2)}[indexPath.row];
            ((void (*)(id, SEL))[self methodForSelector: action])(self, action);
        }

It's got everything you love about C and Objective-C: array literals, function
pointers, casting, selectors and something called [IMPs][IMP tutorial].

This piece of code maps selected cells to methods by putting methods by
indexing selectors in a C array.

Why all the fuss on the second line? wouldn't a simple `performSelector:`
work? The short answer is: to show the compiler that we are responsible
adults. You can read more about it [here](SO Answer);

[IMP tutorial]: <http://www.cocoawithlove.com/2008/02/imp-of-current-method.html>
[SO Answer]: <http://stackoverflow.com/a/20058585/243798>
