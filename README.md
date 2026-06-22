PixelForge — Простейший графический редактор на 7 языках
PixelForge — коллекция из семи независимых реализаций простейшего растрового редактора. Каждая версия работает на своём языке программирования и предлагает базовый набор инструментов для рисования, редактирования и сохранения изображений.

✨ Общие возможности
🖌️ Рисование свободной линией (карандаш/кисть)

🎨 Выбор цвета из палитры или RGB-ввод

📏 Настройка толщины линии

🧽 Ластик (стирание до фона)

🔲 Рисование фигур: прямоугольник, эллипс, линия (в некоторых версиях)

↩️ Отмена/Повтор (Undo/Redo) — до 20 шагов

💾 Сохранение в PNG (или загрузка на сервер)

🖥️ Интерфейсы:

Десктопные GUI: Python (Tkinter), Java (Swing), C# (WinForms)

Веб-приложения: JavaScript (чистый HTML+Canvas), Go, Rust, PHP (сервер + клиент)

📋 Сравнение реализаций
Язык	Интерфейс	Undo/Redo	Фигуры	Сохранение	Экспорт PNG
Python	Tkinter GUI	✅	✅	✅	✅
JavaScript	Веб (Canvas)	✅	✅	❌	✅ (скачать)
Go	Веб (сервер)	❌ (клиент)	✅	✅ (сервер)	✅
Rust	Веб (сервер)	❌ (клиент)	✅	✅ (сервер)	✅
Java	Swing GUI	✅	✅	✅	✅
C#	WinForms GUI	✅	✅	✅	✅
PHP	Веб (сервер)	❌ (клиент)	✅	✅ (сервер)	✅
🚀 Быстрый старт
Python
bash
# Tkinter встроен
python paint.py
JavaScript
Откройте paint.html в браузере.

Go
bash
go run paint.go
# Откройте http://localhost:8080
Rust
bash
cargo run
# Откройте http://localhost:8000
Java
bash
javac PaintApp.java && java PaintApp
C#
bash
csc /reference:System.Windows.Forms.dll /reference:System.Drawing.dll PaintApp.cs
PaintApp.exe
PHP
bash
php -S localhost:8000
# Откройте http://localhost:8000/paint.php
