"use strict";
// index.ts
// Define a function to create and append a heading element with the given text to the body
function createHeading(text) {
    const heading = document.createElement("h1");
    heading.textContent = text;
    document.body.appendChild(heading);
}
// Call the function with the desired text
createHeading("Hello, World!");
