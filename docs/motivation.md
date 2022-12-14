# Why Build This?

So your first thought is most likely, why would I need this? Why would someone take the time to build this? Who cares about dictionaries?

Turns out, a lot of people. Amazing products like [freedict](https://freedict.org), [Wiktionary](https://wiktionary.org), and [Linguee](https://linguee.com) are aimed at providing people with completely free dictionaries to aid NLP research, language learning, and just share knowledge. The only issue is... this data is either only accessible online, not machine readable, not well structured, or some combination of the three.

ODict was developed out of the need for a fast, readily available, and open solution to cross-platform dictionary formats such as Apple Dictionaries, Babylon, Dictd, StarDict and others. 

Current formats are specifically designed to work with specific applications (usually developed by the same company that made the format), and as a result are somewhat uni-directional (there is official documentation on how to write a dictionary in their format, but not on how to read one). This forces users to write dictionaries that only work with one specific dictionary app.

Certain formats, like StarDict or Slab, can be read by multiple dictionary apps, but like most dictionary formats are HTML-based. The data inside is not structured, and the HTML markup might be inconsistent between dictionaries.

_Wouldn't it be nice if there was a completely open-source format, with documentation on both reading and writing the format, that anyone could use in **any** dictionary app?_

That's where ODict comes in.