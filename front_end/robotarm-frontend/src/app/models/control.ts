import {DynamicSvg, MovingSvg} from './dynamic-svg'

export class Control {
    part: string;
    currentValue: number;
    previousValue: number;
    imageSrc: DynamicSvg;

    constructor
        (
            part: string,
            currentValue: number,
            previousValue: number,
            imageSrc: DynamicSvg
        )
    {
        this.part = part;
        this.currentValue = currentValue;
        this.previousValue = previousValue;
        this.imageSrc = imageSrc;
    }

    static default() : Control{ 
        return new Control('', 0, 0, new DynamicSvg("", "", "", false, []))
    }

    static InitalControls() : Control[] {
        return [
            {
                part: "Claw",
                currentValue: 50,
                previousValue: 50,
                imageSrc: new DynamicSvg(
                    "../../assets/Claw_s.svg",
                    "50%",
                    "15%",
                    true,
                    [
                        new MovingSvg("../../assets/Claw_m1.svg", 40, false),
                        new MovingSvg("../../assets/Claw_m2.svg", 40, true),
                    ]
                )
            },
            {
                part: "Hand",
                currentValue: 50,
                previousValue: 50,
                imageSrc: new DynamicSvg(
                    "../../assets/Hand_s.svg",
                    "50%",
                    "40%",
                    false,
                    [new MovingSvg("../../assets/Hand_m.svg", 60, true)]
                )
            },
            {
                part: "Forearm",
                currentValue: 50,
                previousValue: 50,
                imageSrc: new DynamicSvg(
                    "../../assets/Forearm_s.svg",
                    "50%",
                    "5%",
                    true,
                    [new MovingSvg("../../assets/Forearm_m.svg", 50, true)]
                )
            },
            {
                part: "Strongarm",
                currentValue: 50,
                previousValue: 50,
                imageSrc: new DynamicSvg(
                    "../../assets/Strongarm_s.svg",
                    "55%",
                    "64%",
                    true,
                    [new MovingSvg("../../assets/Strongarm_m.svg", 60, true)]
                )
            },
            {
                part: "Shoulder",
                currentValue: 50,
                previousValue: 50,
                imageSrc: new DynamicSvg(
                    "../../assets/Shoulder_s.svg",
                    "69%",
                    "50%",
                    false,
                    [new MovingSvg("../../assets/Shoulder_m.svg", 180, true)]
                )
            }
        ];
    }
}