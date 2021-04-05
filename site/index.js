import {EditorView, basicSetup} from "@codemirror/basic-setup"
import {keymap} from "@codemirror/view"
import {EditorState, Compartment} from "@codemirror/state"
import {defaultTabBinding} from "@codemirror/commands"

import("./node_modules/playground/playground.js").then((playground) => {
    let tabSize = new Compartment;

    let state = EditorState.create({
        doc: "print \"Hello, Passerine!\"",
        extensions: [
            basicSetup,
            keymap.of([defaultTabBinding]),
            tabSize.of(EditorState.tabSize.of(4)),
        ]
    });

    let view = new EditorView({
        state,
        parent: document.body,
    });

    var output = document.createElement('pre');
    output.style = "font-family: monospace;"
    output.innerHTML = "Click 'Run' to run...";
    document.body.appendChild(output);

    var button = document.createElement('Button');
    button.innerHTML = "Run";
    button.addEventListener("click", () => {
        let result = playground.run(view.state.doc.toString());

        if (result === null) {
            output.innerHTML = "Click 'Run' to run...";
        } else {
            output.innerHTML = result;
        }
    })
    document.body.appendChild(button);
});
