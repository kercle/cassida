type ServerMessage = {
    evalResult: {
        input: {
            raw: string,
            latex: string
        },
        output: {
            raw: string,
            latex: string
        }
    }
} | {
    parseError: {
        input: string,
        msg: string,
    }
} | {
    plot: {
        input: {
            raw: string,
            latex: string
        },
        data: []
    }
};
