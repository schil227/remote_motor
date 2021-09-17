export class VideoStream {
    name: string;
    source: string;

    constructor(
        name: string,
        source: string
    )
    {
        this.name = name;
        this.source = source;
    }

    static VideoStreams() : VideoStream[] {
        return [
            {
                name: "Front",
                source: "_front"
            },
            {
                name: "Top",
                source: "_top"
            },
            {
                name: "Side",
                source: "_side"
            }
        ]
    }

    static VideojsStreams() : VideoStream[] {
      return [
          {
              name: "Front",
              source: "_front"
          }
      ]
  }
}
