import init, { concat } from "./pkg/concat.js";

export function concatenateText() {
    var textArea1 = document.getElementById("textArea1").value;
    var textArea2 = document.getElementById("textArea2").value;
    let a1 = [textArea1];
    let a2 = [textArea2];
        init().then(() => {
            document.getElementById("resultTextArea").value =  concat(a1, a2).join('---');
            // document.getElementById("resultTextArea").value =  "HELLO";
        });
}

document.addEventListener('DOMContentLoaded', (event) => {
    document.getElementById("concatenateButton").addEventListener("click", concatenateText);
});