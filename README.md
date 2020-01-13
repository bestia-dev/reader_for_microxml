# reader for microXml

*Things are changing fast. This is the situation on 2020-01-13. LucianoBestia*  

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my templates.  
I found the existance of a standard (or proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case. I will use it for a html templating. This are small files with simple html, that is microXml compatible.  
<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>
TODO: make it more efficient: &str instead of String everywhere

## ChangeLog

2020-01-13 examples
