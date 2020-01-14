# reader for microXml

*Things are changing fast. This is the situation on 2020-01-13. LucianoBestia*  

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my simple html templates.  
I found the existance of a standard (or proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case. This are small simple html files , that are microXml compatible. The speed is not really important. But the size of the code is, because it will be used in WebAssembly. Everything is too big for Wasm.  
No dependencies.  


## References

<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>
<https://github.com/tafia/quick-xml>  
<https://github.com/RazrFalcon/roxmltree>  

## ChangeLog

2020-01-13 examples
2020-01-13 &str instead of String everywhere. No allocation.
