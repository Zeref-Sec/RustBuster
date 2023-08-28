# RustBuster
Web Application fuzzing tool written in rust. Similar functionality to dirbuster.

Usage: `RustBuster <url> <wordlist> [options]`

Options:

  `-sc` : Filter by Status code
  
  `-fl` : Filter by Content length

Example Usage:
`RustBuster https://example.com/ wordlist.txt -sc 404,403 -fl 192`

# To Do
- Add API Fuzzing Capability
- Add HTTP Method Selection
- Add Exception for SSL/HTTPS
- Add Subdomain Fuzzing
  
