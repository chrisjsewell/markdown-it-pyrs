The scheme http will be inserted automatically:
<https://github.github.com/gfm/#example-622>
.
www.commonmark.org
.
<p><a href="http://www.commonmark.org">www.commonmark.org</a></p>
.

After a valid domain, zero or more non-space non-< characters may follow:
<https://github.github.com/gfm/#example-623>
.
Visit www.commonmark.org/help for more information.
.
<p>Visit <a href="http://www.commonmark.org/help">www.commonmark.org/help</a> for more information.</p>
.

Trailing punctuation (specifically, ?, !, ., ,, :, *, _, and ~) will not be considered part of the autolink, though they may be included in the interior of the link:
<https://github.github.com/gfm/#example-624>
.
Visit www.commonmark.org.

Visit www.commonmark.org/a.b.
.
<p>Visit <a href="http://www.commonmark.org">www.commonmark.org</a>.</p>
<p>Visit <a href="http://www.commonmark.org/a.b">www.commonmark.org/a.b</a>.</p>
.

When an autolink ends in `)`, we scan the entire autolink for the total number of parentheses. If there is a greater number of closing parentheses than opening ones, we don’t consider the unmatched trailing parentheses part of the autolink, in order to facilitate including an autolink inside a parenthesis:
<https://github.github.com/gfm/#example-625>
.
www.google.com/search?q=Markup+(business)

www.google.com/search?q=Markup+(business)))

(www.google.com/search?q=Markup+(business))

(www.google.com/search?q=Markup+(business)
.
<p><a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
<p><a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>))</p>
<p>(<a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a>)</p>
<p>(<a href="http://www.google.com/search?q=Markup+(business)">www.google.com/search?q=Markup+(business)</a></p>
.

This check is only done when the link ends in a closing parentheses ), so if the only parentheses are in the interior of the autolink, no special rules are applied:
<https://github.github.com/gfm/#example-626>
.
www.google.com/search?q=(business))+ok
.
<p><a href="http://www.google.com/search?q=(business))+ok">www.google.com/search?q=(business))+ok</a></p>
.

If an autolink ends in a semicolon (;), we check to see if it appears to resemble an entity reference; if the preceding text is & followed by one or more alphanumeric characters. If so, it is excluded from the autolink:
<https://github.github.com/gfm/#example-627>
.
www.google.com/search?q=commonmark&hl=en

www.google.com/search?q=commonmark&hl;
.
<p><a href="http://www.google.com/search?q=commonmark&amp;hl=en">www.google.com/search?q=commonmark&amp;hl=en</a></p>
<p><a href="http://www.google.com/search?q=commonmark">www.google.com/search?q=commonmark</a>&amp;hl;</p>
.

< immediately ends an autolink.
<https://github.github.com/gfm/#example-628>
.
www.commonmark.org/he<lp
.
<p><a href="http://www.commonmark.org/he">www.commonmark.org/he</a>&lt;lp</p>
.

An extended url autolink will be recognised when one of the schemes http://, or https://, followed by a valid domain, then zero or more non-space non-< characters according to extended autolink path validation:
<https://github.github.com/gfm/#example-629>
.
http://commonmark.org

(Visit https://encrypted.google.com/search?q=Markup+(business))
.
<p><a href="http://commonmark.org">http://commonmark.org</a></p>
<p>(Visit <a href="https://encrypted.google.com/search?q=Markup+(business)">https://encrypted.google.com/search?q=Markup+(business)</a>)</p>
.

An extended email autolink will be recognised when an email address is recognised within any text node. Email addresses are recognised according to the following rules:

- One ore more characters which are alphanumeric, or ., -, _, or +.
- An @ symbol.
- One or more characters which are alphanumeric, or - or _, separated by periods (.). There must be at least one period. The last character must not be one of - or _.

The scheme mailto: will automatically be added to the generated link:
<https://github.github.com/gfm/#example-630>
.
foo@bar.baz
.
<p><a href="mailto:foo@bar.baz">foo@bar.baz</a></p>
.

+ can occur before the @, but not after.
<https://github.github.com/gfm/#example-631>
.
hello@mail+xyz.example isn't valid, but hello+xyz@mail.example is.
.
<p>hello@mail+xyz.example isn't valid, but <a href="mailto:hello+xyz@mail.example">hello+xyz@mail.example</a> is.</p>
.

`.`, `-`, and `_` can occur on both sides of the `@`, but only `.` may occur at the end of the email address, in which case it will not be considered part of the address:
<https://github.github.com/gfm/#example-632>
TODO `_` is not supported yet (overriden by emphasis).
.
a.b-c-d@a.b

a.b-c-d@a.b.

a.b-c-d@a.b-

a.b-c-d@a.b_
.
<p><a href="mailto:a.b-c-d@a.b">a.b-c-d@a.b</a></p>
<p><a href="mailto:a.b-c-d@a.b">a.b-c-d@a.b</a>.</p>
<p>a.b-c-d@a.b-</p>
<p>a.b-c-d@a.b_</p>
.

An extended protocol autolink will be recognised when a protocol is recognised within any text node. Valid protocols are:
- `mailto:`
- `xmpp:`
The scheme of the protocol will automatically be added to the generated link. All the rules of email address autolinking apply.
<https://github.github.com/gfm/#example-633>
.
mailto:foo@bar.baz

mailto:a.b-c_d@a.b

mailto:a.b-c_d@a.b.

mailto:a.b-c_d@a.b/

mailto:a.b-c_d@a.b-

mailto:a.b-c_d@a.b_

xmpp:foo@bar.baz

xmpp:foo@bar.baz.
.
<p><a href="mailto:foo@bar.baz">mailto:foo@bar.baz</a></p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a></p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>.</p>
<p><a href="mailto:a.b-c_d@a.b">mailto:a.b-c_d@a.b</a>/</p>
<p>mailto:a.b-c_d@a.b-</p>
<p>mailto:a.b-c_d@a.b_</p>
<p><a href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a></p>
<p><a href="xmpp:foo@bar.baz">xmpp:foo@bar.baz</a>.</p>
.

A described in the specification xmpp offers an optional `/` followed by a resource.
The resource can contain all alphanumeric characters, as well as `@` and `.`.
<https://github.github.com/gfm/#example-634>
.
xmpp:foo@bar.baz/txt

xmpp:foo@bar.baz/txt@bin

xmpp:foo@bar.baz/txt@bin.com
.
<p><a href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a></p>
<p><a href="xmpp:foo@bar.baz/txt@bin">xmpp:foo@bar.baz/txt@bin</a></p>
<p><a href="xmpp:foo@bar.baz/txt@bin.com">xmpp:foo@bar.baz/txt@bin.com</a></p>
.

Further `/` characters are not considered part of the domain:
.
xmpp:foo@bar.baz/txt/bin
.
<p><a href="xmpp:foo@bar.baz/txt">xmpp:foo@bar.baz/txt</a>/bin</p>
.
