// подключить css
var cssId = 'myCss';  // you could encode the css path itself to generate id..
if (!document.getElementById(cssId))
{
    var head  = document.getElementsByTagName('head')[0];
    var link  = document.createElement('link');
    link.id   = cssId;
    link.rel  = 'stylesheet';
    link.type = 'text/css';
    link.href = 'load_screen/load_screen.css';
    link.media = 'all';
    head.appendChild(link);
}

// создать элемент
function appendText() {
    var txt1 = "<div id='loading_screen' class='ring'><span></span></div>"
    $("body").append(txt1);      // Append the new elements
}

appendText()

// убрать элемент плавно
setTimeout(function(){
    var el = $('#loading_screen.ring');
    if (parseFloat(el.css("opacity")) > 0) {
        setInterval(function () {
            if (parseFloat(el.css("opacity")) > 0) {
                el.css("opacity", el.css("opacity") - 0.1);
            };
            if (parseFloat(el.css("opacity")) <= 0) {
                el.remove();
                return
            }
        }, 70);
    }
},5000);