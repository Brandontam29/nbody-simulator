/**
 * @author mrdoob / http://mrdoob.com/
 */

export class Stats {
  private mode: number = 0;
  public dom: HTMLDivElement;
  private fpsPanel: StatsPanel;
  private msPanel: StatsPanel;
  private memPanel: StatsPanel | null = null;
  private beginTime: number;
  private prevTime: number;
  private frames: number = 0;

  // Backwards compatibility
  public domElement: HTMLDivElement;

  constructor() {
    this.dom = document.createElement('div');
    this.dom.style.cssText = 'position:fixed;top:0;left:0;cursor:pointer;opacity:0.9;z-index:10000';
    this.dom.addEventListener(
      'click',
      (event) => {
        event.preventDefault();
        this.showPanel(++this.mode % this.dom.children.length);
      },
      false,
    );

    this.fpsPanel = this.addPanel(new StatsPanel('FPS', '#0ff', '#002'));
    this.msPanel = this.addPanel(new StatsPanel('MS', '#0f0', '#020'));

    if (self.performance && (self.performance as any).memory) {
      this.memPanel = this.addPanel(new StatsPanel('MB', '#f08', '#201'));
    }

    this.showPanel(0);

    this.beginTime = (performance || Date).now();
    this.prevTime = this.beginTime;

    this.domElement = this.dom;
  }

  addPanel(panel: StatsPanel): StatsPanel {
    this.dom.appendChild(panel.dom);
    return panel;
  }

  showPanel(id: number): void {
    for (let i = 0; i < this.dom.children.length; i++) {
      (this.dom.children[i] as HTMLElement).style.display = i === id ? 'block' : 'none';
    }
    this.mode = id;
  }

  begin(): void {
    this.beginTime = (performance || Date).now();
  }

  end(): number {
    this.frames++;
    const time = (performance || Date).now();
    this.msPanel.update(time - this.beginTime, 200);

    if (time >= this.prevTime + 1000) {
      this.fpsPanel.update((this.frames * 1000) / (time - this.prevTime), 100);
      this.prevTime = time;
      this.frames = 0;

      if (this.memPanel) {
        const memory = (performance as any).memory;
        this.memPanel.update(memory.usedJSHeapSize / 1048576, memory.jsHeapSizeLimit / 1048576);
      }
    }
    return time;
  }

  update(): void {
    this.beginTime = this.end();
  }

  setMode(id: number): void {
    this.showPanel(id);
  }
}

class StatsPanel {
  private min: number = Infinity;
  private max: number = 0;
  public dom: HTMLCanvasElement;
  private context: CanvasRenderingContext2D;
  private name: string;
  private fg: string;
  private bg: string;
  private PR: number;
  private WIDTH: number;
  private HEIGHT: number;
  private TEXT_X: number;
  private TEXT_Y: number;
  private GRAPH_X: number;
  private GRAPH_Y: number;
  private GRAPH_WIDTH: number;
  private GRAPH_HEIGHT: number;

  constructor(name: string, fg: string, bg: string) {
    this.name = name;
    this.fg = fg;
    this.bg = bg;
    this.PR = Math.round(window.devicePixelRatio || 1);
    this.WIDTH = 80 * this.PR;
    this.HEIGHT = 48 * this.PR;
    this.TEXT_X = 3 * this.PR;
    this.TEXT_Y = 2 * this.PR;
    this.GRAPH_X = 3 * this.PR;
    this.GRAPH_Y = 15 * this.PR;
    this.GRAPH_WIDTH = 74 * this.PR;
    this.GRAPH_HEIGHT = 30 * this.PR;

    this.dom = document.createElement('canvas');
    this.dom.width = this.WIDTH;
    this.dom.height = this.HEIGHT;
    this.dom.style.cssText = 'width:80px;height:48px';

    this.context = this.dom.getContext('2d')!;
    this.context.font = 'bold ' + 9 * this.PR + 'px Helvetica,Arial,sans-serif';
    this.context.textBaseline = 'top';

    this.context.fillStyle = bg;
    this.context.fillRect(0, 0, this.WIDTH, this.HEIGHT);

    this.context.fillStyle = fg;
    this.context.fillText(name, this.TEXT_X, this.TEXT_Y);
    this.context.fillRect(this.GRAPH_X, this.GRAPH_Y, this.GRAPH_WIDTH, this.GRAPH_HEIGHT);

    this.context.fillStyle = bg;
    this.context.globalAlpha = 0.9;
    this.context.fillRect(this.GRAPH_X, this.GRAPH_Y, this.GRAPH_WIDTH, this.GRAPH_HEIGHT);
  }

  update(value: number, maxValue: number): void {
    this.min = Math.min(this.min, value);
    this.max = Math.max(this.max, value);

    this.context.fillStyle = this.bg;
    this.context.globalAlpha = 1;
    this.context.fillRect(0, 0, this.WIDTH, this.GRAPH_Y);
    this.context.fillStyle = this.fg;
    this.context.fillText(
      Math.round(value) +
        ' ' +
        this.name +
        ' (' +
        Math.round(this.min) +
        '-' +
        Math.round(this.max) +
        ')',
      this.TEXT_X,
      this.TEXT_Y,
    );

    this.context.drawImage(
      this.dom,
      this.GRAPH_X + this.PR,
      this.GRAPH_Y,
      this.GRAPH_WIDTH - this.PR,
      this.GRAPH_HEIGHT,
      this.GRAPH_X,
      this.GRAPH_Y,
      this.GRAPH_WIDTH - this.PR,
      this.GRAPH_HEIGHT,
    );

    this.context.fillRect(
      this.GRAPH_X + this.GRAPH_WIDTH - this.PR,
      this.GRAPH_Y,
      this.PR,
      this.GRAPH_HEIGHT,
    );

    this.context.fillStyle = this.bg;
    this.context.globalAlpha = 0.9;
    this.context.fillRect(
      this.GRAPH_X + this.GRAPH_WIDTH - this.PR,
      this.GRAPH_Y,
      this.PR,
      Math.round((1 - value / maxValue) * this.GRAPH_HEIGHT),
    );
  }
}
