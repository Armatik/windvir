const scale = 180;

class Drawable {
    constructor (color = [0,0,0]) {
        this.rgb = color;
    }

    getCSS_RGB() {
        return `rgb(${this.rgb[0]*255} ${this.rgb[1]*255} ${this.rgb[2]*255})`;
    }

    static convertXIntoGliumCoordinate(x) {

        return 2*x*window.innerHeight/window.innerHeight/window.innerHeight/scale; 
    }

    static convertYIntoGliumCoordinate(y) {
        return -2*y/window.innerHeight/scale; 
    }

    static convertGliumXIntoCanvasCoordinate(x) {
        return x*window.innerHeight*window.innerHeight/window.innerHeight/2*scale; 
    }

    static convertGliumYIntoCanvasCoordinate(y) {
        return -y*window.innerHeight/2*scale; 
    }
}

class Fillable extends Drawable {
    constructor (color = [0,0,0], fill_mode = false) {
        super(color);
        this.is_fill = fill_mode;
    }
}

class Circle extends Fillable {
    constructor(x, y, radius,color = [0,0,0], fill_mode = false) {
        super(color,fill_mode);
        this.x = x;
        this.y = y;
        this.radius = radius;
    }

    draw() {
        context.save();
        // context.arc()
        context.restore();
    }
}

class Rectangle extends Fillable {
    constructor(sx, sy, ex, ey, color = [0,0,0], fill_mode = false) {
        super(color,fill_mode);
        this.left_up_angle_x = sx;
        this.left_up_angle_y = sy;
        this.right_down_angle_x = ex;
        this.right_down_angle_y = ey;
    }

    draw() {
        context.save();
        context.rect(
            Drawable.convertGliumXIntoCanvasCoordinate(this.left_up_angle_x),
            Drawable.convertGliumYIntoCanvasCoordinate(this.left_up_angle_y),
            Drawable.convertGliumXIntoCanvasCoordinate(this.right_down_angle_x - this.left_up_angle_x),
            Drawable.convertGliumYIntoCanvasCoordinate(this.right_down_angle_y - this.left_up_angle_y)
            );

        if (this.is_fill) context.fill();
        else context.stroke();
        context.restore();
    }
}

class Polygon extends Fillable {
    constructor(points = [], color = [0,0,0], fill_mode = false) {
        super(color, fill_mode);
        this.points = points;
    }
    draw() {
        context.save();
        context.beginPath();
        context.moveTo(
            Drawable.convertGliumXIntoCanvasCoordinate(this.points[0]),
            Drawable.convertGliumYIntoCanvasCoordinate(this.points[1])
        );
        for (let point of this.points) {
            context.lineTo(
                Drawable.convertGliumXIntoCanvasCoordinate(point[0]),
                Drawable.convertGliumYIntoCanvasCoordinate(point[1])
            );
        }
        context.closePath();
        if (this.is_fill) context.fill();
        else context.stroke();
        context.restore();
    }
}

class Segment extends Drawable {
    constructor(startX,startY, color = [0,0,0]) {
        super(color);
        this.p0_x = startX;
        this.p0_y = startY;
        this.p1_x = startX;
        this.p1_y = startY;
    }
    draw() {
        context.save();
        context.beginPath();
        context.moveTo(
            Drawable.convertGliumXIntoCanvasCoordinate(this.p0_x),
            Drawable.convertGliumYIntoCanvasCoordinate(this.p0_y)
        );
        context.lineTo(
            Drawable.convertGliumXIntoCanvasCoordinate(this.p1_x),
            Drawable.convertGliumYIntoCanvasCoordinate(this.p1_y)
        );
        context.stroke();
        context.restore();
    }
}

function resizeCanvas(canvas) {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

function initContext(context) {
    context.font = '14px sans-serif';
    context.translate(context.canvas.width/2,context.canvas.height/2);
}

function updateData() {

}

function renderFrame() {
    context.clearRect(-innerWidth/2,-innerHeight/2,innerWidth,innerHeight);
    if (currentObject !== null) currentObject.draw();
    for (let polygon of outputData.polygons) {
        if (polygon !== null) polygon.draw();
    }
    for (let line of outputData.lines) {
        if (line !== null) line.draw();
    }
    switch (currentMode) {
        case Mode.POLYGON:
            context.fillText("Полигионы",-innerWidth/2,-innerHeight/2 + 14);
            break;
        case Mode.LINE:
            context.fillText("Отрезки",-innerWidth/2,-innerHeight/2 + 14);
            break;
    }
    requestAnimationFrame(renderFrame);
}

const outputData = {
    circles: [],
    rectangles: [],
    polygons: [],
    lines: []
};

const Mode = {
    POLYGON: 0,
    LINE: 1
}

let currentMode = Mode.POLYGON;
let currentObject = null;

addEventListener("keydown",
    (keyboard) => {
        switch (keyboard.key) {
        case "Enter":
            if (currentMode === Mode.POLYGON) {
                currentObject.points.pop();
                outputData.polygons.push(currentObject);
                currentObject = null;
            }
            break;
        case "[":
            currentMode = (++currentMode)%2;
            currentObject = null;
            break;
        }
    }
);

addEventListener("mousedown",
    (mouse) => {
        const x = Drawable.convertXIntoGliumCoordinate(mouse.clientX - window.innerWidth/2);
        const y = Drawable.convertYIntoGliumCoordinate(mouse.clientY - window.innerHeight/2);
        if(currentObject == null) {
            switch (currentMode) {
                case Mode.POLYGON: 
                    currentObject = new Polygon([[x,y],[x,y]]);
                    break;
                case Mode.LINE: 
                    currentObject = new Segment(x,y);
                    break;
            }
        } else {
            switch (currentMode) {
                case Mode.POLYGON: 
                    currentObject.points.push([x,y]);
                    break;
                case Mode.LINE: 
                    currentObject.p1_x = x;
                    currentObject.p1_y = y;
                    outputData.lines.push(currentObject);
                    currentObject = null;
                    break;
            }
        }
    }
);

addEventListener("mousemove",
    (mouse) => {
        const x = Drawable.convertXIntoGliumCoordinate(mouse.clientX - window.innerWidth/2);
        const y = Drawable.convertYIntoGliumCoordinate(mouse.clientY - window.innerHeight/2);
        if(currentObject !== null) {
            switch (currentMode) {
                case Mode.POLYGON: 
                    currentObject.points[currentObject.points.length - 1][0] = x;
                    currentObject.points[currentObject.points.length - 1][1] = y;
                    break;
                case Mode.LINE: 
                    currentObject.p1_x = x;
                    currentObject.p1_y = y;
                    break;
            }
        }
    }
);

// (function main() {
    const canvas = document.querySelector("#canvas");
    resizeCanvas(canvas);
    const context = canvas.getContext("2d");
    initContext(context);
    renderFrame();
    // console.log(JSON.stringify(outputData));
// })()