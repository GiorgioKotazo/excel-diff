# Excel Diff

**Compare two Excel sheets and get clean, formula-free results.**

Excel Diff is a lightweight command-line tool that reads two Excel files, compares records from the columns you specify, and produces a single `.xlsx` report showing which rows exist only in one file or the other.

No formulas. No broken links. No dependencies on Microsoft Excel being installed. Just a fast, portable utility that works on Windows, macOS, and Linux.

---

## What It Does

- Reads two Excel (`.xlsx`) files.
- Compares values from two columns in each file.
- Identifies rows that are **only in File 1** and rows that are **only in File 2**.
- Saves the result as a new `.xlsx` file with two plain tables — no formulas, no external links.

---

## Requirements

### Pre-built binaries
- Windows 10/11 (x64)

### Building from source
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Any OS that Rust supports: Windows, macOS, Linux

---

## Installation

### Option 1: Download a pre-built binary (Windows)

1. Download `excel-diff.exe` from the [Releases](https://github.com/yourname/excel-diff/releases) page.
2. Place it in the folder where your Excel files are located (or add it to your `PATH`).

### Option 2: Build from source

```bash
# Clone the repository
git clone https://github.com/yourname/excel-diff.git
cd excel-diff

# Build a release binary
cargo build --release

# The compiled binary will be at:
#   ./target/release/excel-diff        (Linux / macOS)
#   .\target\release\excel-diff.exe     (Windows)
```

---

## Usage

```bash
excel-diff <file1> <sheet1> <name_col1> <num_col1> <file2> <sheet2> <name_col2> <num_col2> [output.xlsx]
```

### Arguments

| # | Argument | Description |
|---|----------|-------------|
| 1 | `file1` | Path to the first Excel file |
| 2 | `sheet1` | Sheet name in the first file |
| 3 | `name_col1` | Column letter for the primary key / name (e.g., `A`, `D`) |
| 4 | `num_col1` | Column letter for the secondary value / number (e.g., `B`, `E`) |
| 5 | `file2` | Path to the second Excel file |
| 6 | `sheet2` | Sheet name in the second file |
| 7 | `name_col2` | Column letter for the primary key in the second file |
| 8 | `num_col2` | Column letter for the secondary value in the second file |
| 9 | `output.xlsx` | *(Optional)* Output file name. Defaults to `diff_result.xlsx` |

> **Tip:** Column letters are case-insensitive (`d` and `D` are the same).

---

## Examples

### Linux / macOS

```bash
./excel-diff \
  "example_1.xlsx" "list_1" D E \
  "example_2.xlsx" "list_2" D E \
  result.xlsx
```

### Windows (Command Prompt / PowerShell)

```powershell
.\excel-diff.exe "example_1.xlsx" "list_1" D E "example_2.xlsx" "list_2" D E result.xlsx
```

### Windows (Batch file)

Create a file named `run_diff.bat` next to your Excel files:

```bat
excel-diff.exe "example_1.xlsx" "list_1" D E "example_2.xlsx" "list_2" D E result.xlsx
pause
```

Then simply **double-click** `run_diff.bat`. A new `result.xlsx` will appear in the same folder within seconds.

---

## What's Inside the Result

The output file contains a single sheet named `Diff` with two tables stacked vertically:

1. **Top table** — rows that exist **only in File 1**.  
   Header includes the original file name and sheet name so you never lose track of the source.

2. **Bottom table** — rows that exist **only in File 2**.  
   Same clear header with file and sheet references.

Both tables are plain text values. You can open the result on any computer without worrying about broken formulas or missing links.

---

## Who Is This For?

- **Data analysts** who compare lists, personnel records, or project data between two Excel files.
- **Office workers** who need a quick diff report without learning advanced Excel functions.
- **IT admins** who want a portable, dependency-free tool that runs on any OS.

---

## Development

### Running checks locally

```bash
make check   # formatting + clippy
make test    # run tests
make build   # release build
make all     # check + test
```

---

## License

MIT — feel free to use, modify, and distribute.
