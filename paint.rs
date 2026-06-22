// paint.rs - Графический редактор на Rust (веб-сервер с HTML+JS)
// Зависимости: actix-web, actix-files
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_web::http::header;
use std::fs;
use base64::{engine::general_purpose, Engine};

async fn index() -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html><head><meta charset="UTF-8"><title>🎨 PixelForge - Rust</title>
<style>body{font-family:sans-serif;background:#2c3e50;display:flex;justify-content:center;align-items:center;min-height:100vh;margin:0;}
.container{background:#ecf0f1;padding:20px;border-radius:16px;box-shadow:0 10px 30px rgba(0,0,0,0.3);}
.toolbar{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px;align-items:center;}
.toolbar button{padding:6px 12px;border:none;border-radius:6px;background:#3498db;color:white;cursor:pointer;}
.toolbar button:hover{background:#2980b9;}
.toolbar input[type="color"]{width:40px;height:40px;border:none;cursor:pointer;}
.toolbar input[type="range"]{width:100px;}
canvas{border:2px solid #bdc3c7;border-radius:8px;cursor:crosshair;background:white;}
.status{margin-top:10px;display:flex;justify-content:space-between;font-size:14px;color:#2c3e50;}
</style></head>
<body>
<div class="container">
<h2>🎨 PixelForge · Rust</h2>
<div class="toolbar">
<button onclick="setTool('brush')">🖌️ Кисть</button>
<button onclick="setTool('eraser')">🧽 Ластик</button>
<button onclick="setTool('rect')">▭ Прямоугольник</button>
<button onclick="setTool('ellipse')">⬤ Эллипс</button>
<button onclick="setTool('line')">╱ Линия</button>
<input type="color" id="colorPicker" value="#000000">
<label>Толщина: <input type="range" id="sizeSlider" min="1" max="20" value="5"></label>
<button onclick="undo()">↩️ Отменить</button>
<button onclick="redo()">↪️ Повторить</button>
<button onclick="clearCanvas()">🗑️ Очистить</button>
<button onclick="saveImage()">💾 Сохранить</button>
</div>
<canvas id="canvas" width="800" height="600"></canvas>
<div class="status"><span>Инструмент: <span id="toolDisplay">Кисть</span></span><span>Размер: <span id="sizeDisplay">5</span></span></div>
</div>
<script>
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');
const colorPicker = document.getElementById('colorPicker');
const sizeSlider = document.getElementById('sizeSlider');

let isDrawing=false, lastX=0, lastY=0, tool='brush', color='#000000', size=5;
let startX=0,startY=0, undoStack=[], redoStack=[], MAX_UNDO=20, tempShape=null;

function init(){ ctx.fillStyle='#ffffff'; ctx.fillRect(0,0,canvas.width,canvas.height); pushUndo(); updateDisplay(); }
function pushUndo(){ undoStack.push(canvas.toDataURL()); if(undoStack.length>MAX_UNDO) undoStack.shift(); redoStack=[]; }
function restoreFromDataURL(data){ let img=new Image(); img.onload=()=>{ ctx.clearRect(0,0,canvas.width,canvas.height); ctx.drawImage(img,0,0); }; img.src=data; }
function undo(){ if(undoStack.length<2)return; redoStack.push(undoStack.pop()); restoreFromDataURL(undoStack[undoStack.length-1]); }
function redo(){ if(redoStack.length===0)return; let data=redoStack.pop(); undoStack.push(data); restoreFromDataURL(data); }
function setTool(t){ tool=t; document.getElementById('toolDisplay').textContent=t==='brush'?'Кисть':t==='eraser'?'Ластик':t==='rect'?'Прямоугольник':t==='ellipse'?'Эллипс':'Линия'; }
function clearCanvas(){ ctx.fillStyle='#ffffff'; ctx.fillRect(0,0,canvas.width,canvas.height); pushUndo(); }
function saveImage(){ 
    let link=document.createElement('a'); link.download='drawing.png'; link.href=canvas.toDataURL('image/png'); link.click();
    // также отправляем на сервер
    let dataURL=canvas.toDataURL('image/png');
    fetch('/save', {method:'POST', headers:{'Content-Type':'application/x-www-form-urlencoded'}, body:'image='+encodeURIComponent(dataURL)});
}
function updateDisplay(){ document.getElementById('sizeDisplay').textContent=size; }

function startDrawing(e){ isDrawing=true; let rect=canvas.getBoundingClientRect(); lastX=e.clientX-rect.left; lastY=e.clientY-rect.top; startX=lastX; startY=lastY; tempShape=null; }
function draw(e){ if(!isDrawing)return; let rect=canvas.getBoundingClientRect(); let x=e.clientX-rect.left; let y=e.clientY-rect.top;
if(tool==='brush'||tool==='eraser'){ ctx.beginPath(); ctx.moveTo(lastX,lastY); ctx.lineTo(x,y); ctx.strokeStyle=tool==='eraser'?'#ffffff':color; ctx.lineWidth=size; ctx.lineCap='round'; ctx.stroke(); lastX=x; lastY=y; }
else if(tool==='rect'||tool==='ellipse'||tool==='line'){ if(tempShape){ restoreFromDataURL(undoStack[undoStack.length-1]); } drawShape(startX,startY,x,y); tempShape=true; }
}
function drawShape(x1,y1,x2,y2){ ctx.save(); ctx.strokeStyle=color; ctx.lineWidth=size; if(tool==='rect'){ ctx.strokeRect(Math.min(x1,x2),Math.min(y1,y2),Math.abs(x2-x1),Math.abs(y2-y1));}
else if(tool==='ellipse'){ ctx.beginPath(); ctx.ellipse((x1+x2)/2,(y1+y2)/2,Math.abs(x2-x1)/2,Math.abs(y2-y1)/2,0,0,2*Math.PI); ctx.stroke();}
else if(tool==='line'){ ctx.beginPath(); ctx.moveTo(x1,y1); ctx.lineTo(x2,y2); ctx.stroke();}
ctx.restore(); }
function stopDrawing(e){ if(!isDrawing)return; isDrawing=false; if(tool==='rect'||tool==='ellipse'||tool==='line'){ let rect=canvas.getBoundingClientRect(); let x=e.clientX-rect.left; let y=e.clientY-rect.top; drawShape(startX,startY,x,y); pushUndo(); } else { pushUndo(); } tempShape=null; updateDisplay(); }
canvas.addEventListener('mousedown',startDrawing);
canvas.addEventListener('mousemove',draw);
canvas.addEventListener('mouseup',stopDrawing);
canvas.addEventListener('mouseleave',stopDrawing);
colorPicker.oninput=(e)=>{ color=e.target.value; };
sizeSlider.oninput=(e)=>{ size=parseInt(e.target.value); document.getElementById('sizeDisplay').textContent=size; };
init();
</script>
</body></html>"#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn save(web::Form(form): web::Form<serde_json::Value>) -> HttpResponse {
    if let Some(image_data) = form.get("image").and_then(|v| v.as_str()) {
        if let Some(encoded) = image_data.strip_prefix("data:image/png;base64,") {
            if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                if let Ok(()) = fs::write("drawing.png", decoded) {
                    return HttpResponse::Ok().body("ok");
                }
            }
        }
    }
    HttpResponse::BadRequest().body("error")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/save", web::post().to(save))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
