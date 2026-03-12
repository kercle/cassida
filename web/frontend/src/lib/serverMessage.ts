type ServerMessage = {
    evalResult: {
        input: string,
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
        input: string,
        data: []
    }
} | {
    helpTableOfContents: {
        input: string,
        builtins: Array<[string, string]>
    }
} | {
    helpBuiltin: {
        input: string,
        title: string,
        patterns: [],
    }
};
