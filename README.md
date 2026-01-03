# CLI Stock Visualizeur

### High-Level Architecture

```bash
User
 ↓
Rust CLI (clap)
 ↓  (subprocess / args)
Python analysis script
 ↓
CSV / stdout / images
 ↓
Terminal output + notebook report
```
