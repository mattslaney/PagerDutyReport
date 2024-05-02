function print_line(line: String) {
  const fo = document.getElementById("file_contents") as HTMLInputElement;
  fo.innerHTML += line;
}
