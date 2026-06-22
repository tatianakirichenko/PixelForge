// PaintApp.cs - Графический редактор на C# (WinForms)
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Windows.Forms;

public class PaintApp : Form
{
    private Bitmap bitmap;
    private Graphics g;
    private Point prevPoint, startPoint;
    private Color color = Color.Black;
    private int brushSize = 5;
    private string tool = "brush";
    private bool isDrawing = false;
    private PictureBox pictureBox;

    public PaintApp()
    {
        Text = "🎨 PixelForge - C#";
        Size = new Size(900, 700);
        bitmap = new Bitmap(800, 600);
        using (var g = Graphics.FromImage(bitmap))
            g.Clear(Color.White);

        pictureBox = new PictureBox { Dock = DockStyle.Fill, Image = bitmap, BackColor = Color.White };
        pictureBox.MouseDown += (s, e) => {
            prevPoint = e.Location;
            startPoint = e.Location;
            isDrawing = true;
        };
        pictureBox.MouseMove += (s, e) => {
            if (!isDrawing) return;
            Point cur = e.Location;
            using (var g = Graphics.FromImage(bitmap))
            {
                g.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;
                if (tool == "brush")
                {
                    using (Pen pen = new Pen(color, brushSize)) { pen.StartCap = pen.EndCap = System.Drawing.Drawing2D.LineCap.Round;
                        g.DrawLine(pen, prevPoint, cur); }
                }
                else if (tool == "eraser")
                {
                    using (Pen pen = new Pen(Color.White, brushSize)) { pen.StartCap = pen.EndCap = System.Drawing.Drawing2D.LineCap.Round;
                        g.DrawLine(pen, prevPoint, cur); }
                }
                else if (tool == "rect")
                {
                    // Для упрощения будем рисовать только по завершению
                }
                else if (tool == "ellipse")
                {
                }
                else if (tool == "line")
                {
                }
                prevPoint = cur;
                pictureBox.Invalidate();
            }
        };
        pictureBox.MouseUp += (s, e) => {
            if (tool == "rect" || tool == "ellipse" || tool == "line")
            {
                // Рисуем фигуру
                using (var g = Graphics.FromImage(bitmap))
                {
                    g.SmoothingMode = System.Drawing.Drawing2D.SmoothingMode.AntiAlias;
                    using (Pen pen = new Pen(color, brushSize))
                    {
                        Rectangle rect = GetRectangle(startPoint, e.Location);
                        if (tool == "rect") g.DrawRectangle(pen, rect);
                        else if (tool == "ellipse") g.DrawEllipse(pen, rect);
                        else if (tool == "line") g.DrawLine(pen, startPoint, e.Location);
                    }
                }
                pictureBox.Invalidate();
            }
            isDrawing = false;
        };

        Controls.Add(pictureBox);

        // Toolbar
        ToolStrip toolbar = new ToolStrip();
        string[] tools = { "brush", "eraser", "rect", "ellipse", "line" };
        string[] labels = { "🖌️ Кисть", "🧽 Ластик", "▭ Прямоугольник", "⬤ Эллипс", "╱ Линия" };
        for (int i = 0; i < tools.Length; i++)
        {
            ToolStripButton btn = new ToolStripButton(labels[i]);
            string t = tools[i];
            btn.Click += (s, e) => tool = t;
            toolbar.Items.Add(btn);
        }
        ToolStripButton colorBtn = new ToolStripButton("Цвет");
        colorBtn.Click += (s, e) => {
            ColorDialog cd = new ColorDialog();
            if (cd.ShowDialog() == DialogResult.OK) color = cd.Color;
        };
        toolbar.Items.Add(colorBtn);
        ToolStripLabel sizeLabel = new ToolStripLabel("Толщина:");
        toolbar.Items.Add(sizeLabel);
        NumericUpDown sizeBox = new NumericUpDown { Minimum = 1, Maximum = 20, Value = 5, Width = 40 };
        sizeBox.ValueChanged += (s, e) => brushSize = (int)sizeBox.Value;
        toolbar.Items.Add(new ToolStripControlHost(sizeBox));
        ToolStripButton clearBtn = new ToolStripButton("🗑️ Очистить");
        clearBtn.Click += (s, e) => {
            using (var g = Graphics.FromImage(bitmap))
                g.Clear(Color.White);
            pictureBox.Invalidate();
        };
        toolbar.Items.Add(clearBtn);
        ToolStripButton saveBtn = new ToolStripButton("💾 Сохранить");
        saveBtn.Click += (s, e) => {
            SaveFileDialog sfd = new SaveFileDialog { Filter = "PNG|*.png" };
            if (sfd.ShowDialog() == DialogResult.OK)
                bitmap.Save(sfd.FileName, ImageFormat.Png);
        };
        toolbar.Items.Add(saveBtn);
        Controls.Add(toolbar);
        toolbar.Dock = DockStyle.Top;
    }

    private Rectangle GetRectangle(Point p1, Point p2)
    {
        return new Rectangle(Math.Min(p1.X, p2.X), Math.Min(p1.Y, p2.Y),
                             Math.Abs(p1.X - p2.X), Math.Abs(p1.Y - p2.Y));
    }

    [STAThread]
    static void Main() { Application.EnableVisualStyles(); Application.Run(new PaintApp()); }
}
