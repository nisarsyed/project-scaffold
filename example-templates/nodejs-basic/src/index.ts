/**
 * {{project_name}}
 * {{description}}
 */

function greet(name: string): string {
  return `Hello, ${name}!`;
}

function main(): void {
  const message = greet("World");
  console.log(message);
  console.log(`Running {{project_name}}`);
}

main();
