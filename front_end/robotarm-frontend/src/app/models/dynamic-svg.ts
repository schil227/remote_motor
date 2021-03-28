export class DynamicSvg {
    staticFileName: string;
    xOrigin: string;
    yOrigin: string;
    staticFirst: boolean;
    movingSvgs: MovingSvg[];

    constructor
    (
        staticFileName: string, 
        xOrigin: string,
        yOrigin: string,
        staticFirst: boolean,
        movingSvgs: MovingSvg[]
    ){
        this.staticFileName = staticFileName;
        this.xOrigin = xOrigin;
        this.yOrigin = yOrigin;
        this.staticFirst = staticFirst;
        this.movingSvgs = movingSvgs;
    }
}

export class MovingSvg {
    movingFileName: string;
    rotationDelta: number;
    isClockwise: boolean;

    constructor
    (
        movingFileName: string,
        rotationDelta: number,
        isClockwise: boolean
    ){
        this.movingFileName = movingFileName;
        this.rotationDelta = rotationDelta;
        this.isClockwise = isClockwise;
    }
}