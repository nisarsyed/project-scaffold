"""Main CLI entrypoint."""

from typing import Annotated

import typer
from rich.console import Console
from rich.table import Table

app = typer.Typer(
    help="{{description}}",
    no_args_is_help=True,
    rich_markup_mode="rich",
)
console = Console()


@app.command()
def hello(
    name: Annotated[str, typer.Argument(help="Name to greet")] = "World",
    count: Annotated[int, typer.Option("--count", "-c", help="Number of greetings")] = 1,
    loud: Annotated[bool, typer.Option("--loud", "-l", help="Greet loudly")] = False,
) -> None:
    """Say hello to someone."""
    greeting = f"Hello, {name}!"
    if loud:
        greeting = greeting.upper()
    for _ in range(count):
        console.print(f"[green]{greeting}[/green]")


@app.command()
def info() -> None:
    """Show system information."""
    import platform
    import sys

    table = Table(title="System Information")
    table.add_column("Property", style="cyan")
    table.add_column("Value", style="green")

    table.add_row("Python Version", sys.version.split()[0])
    table.add_row("Platform", platform.system())
    table.add_row("Architecture", platform.machine())

    console.print(table)


@app.command()
def version() -> None:
    """Show the version."""
    from . import __version__

    console.print(f"[bold]{{project_name}}[/bold] version [cyan]{__version__}[/cyan]")


def main() -> None:
    """Entry point for the CLI."""
    app()


if __name__ == "__main__":
    main()
