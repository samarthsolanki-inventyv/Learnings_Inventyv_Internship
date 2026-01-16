// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */

// Size class
export class Size {
  constructor(width = 80, height = 60) {
    this.width = width;
    this.height = height;
  }

  resize(newWidth, newHeight) {
    this.width = newWidth;
    this.height = newHeight;
  }
}

// Position class
export class Position {
  constructor(x = 0, y = 0) {
    this.x = x;
    this.y = y;
  }

  move(newX, newY) {
    this.x = newX;
    this.y = newY;
  }
}

// ProgramWindow class
export class ProgramWindow {
  constructor() {
    this.screenSize = new Size(800, 600);
    this.size = new Size();        // default 80x60
    this.position = new Position(); // default 0x0
  }

  resize(newSize) {
    // Minimum size is 1x1
    const width = Math.max(1, newSize.width);
    const height = Math.max(1, newSize.height);

    // Maximum size depends on current position
    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;

    this.size.width = Math.min(width, maxWidth);
    this.size.height = Math.min(height, maxHeight);
  }

  move(newPosition) {
    // Minimum position is 0x0
    const x = Math.max(0, newPosition.x);
    const y = Math.max(0, newPosition.y);

    // Maximum position depends on current size
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;

    this.position.x = Math.min(x, maxX);
    this.position.y = Math.min(y, maxY);
  }
}

// changeWindow function
export function changeWindow(programWindow) {
  programWindow.resize(new Size(400, 300));
  programWindow.move(new Position(100, 150));
  return programWindow;
}
