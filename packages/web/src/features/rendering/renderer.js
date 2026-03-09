export class Renderer {
    constructor(context) {
        this.ctx = context;
        this.PAINT_SCALE = 2;
    }

    draw(engine, params) {
        const { canvasWidth, canvasHeight, worldWidth } = params;
        const worldHeight = (worldWidth * canvasHeight) / canvasWidth;
        const count = engine.count;
        const views = engine.views;

        this.ctx.clearRect(0, 0, canvasWidth, canvasHeight);

        for (let i = 0; i < count; i++) {
            const x = views.posX[i];
            const y = views.posY[i];

            // Basic Culling
            if (x < -worldWidth * 0.5 || x > worldWidth * 1.5 || y < -worldHeight * 0.5 || y > worldHeight * 1.5) {
                continue;
            }

            const diameter = views.diameters[i];
            const pcX = (x / worldWidth) * canvasWidth;
            const pcY = (y / worldHeight) * canvasHeight;
            const pcDiameter = (diameter / worldWidth) * canvasWidth * this.PAINT_SCALE;

            const r = Math.floor(views.colors[i * 3]);
            const g = Math.floor(views.colors[i * 3 + 1]);
            const b = Math.floor(views.colors[i * 3 + 2]);

            this.ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
            this.ctx.fillRect(
                pcX - pcDiameter / 2,
                pcY - pcDiameter / 2,
                pcDiameter,
                pcDiameter
            );
        }
    }
}
