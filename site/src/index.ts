function createHeading(text: string): void {
    const heading = document.createElement("h1");
    heading.textContent = text;
    document.body.appendChild(heading);
}

createHeading("Hello, World!");

