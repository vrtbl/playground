import {EditorState, EditorView, basicSetup} from "@codemirror/basic-setup"

import("./node_modules/playground/playground.js").then((playground) => {
    let editor = new EditorView({
        state: EditorState.create({
            extensions: [basicSetup]
        }),
        parent: document.body
    })

    playground.run("print \"Hello!\"")
});
