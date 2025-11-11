#!/bin/bash

# Terminal Example - Git workflow and system commands

# Color codes for terminal output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}=== Git Status ===${NC}"
git status

# Output:
# On branch main
# Your branch is up to date with 'origin/main'.
#
# Changes to be committed:
#   (use "git restore --staged <file>..." to unstage)
#         modified:   src/components/Header.tsx
#         new file:   src/utils/api.ts
#
# Changes not staged for commit:
#   (use "git add <file>..." to update what will be committed)
#   (use "git restore <file>..." to discard changes in working directory)
#         modified:   README.md
#
# Untracked files:
#   (use "git add <file>..." to include in what will be committed)
#         .env.local

echo -e "\n${BLUE}=== Git Log ===${NC}"
git log --oneline -5

# Output:
# a1b2c3d (HEAD -> main) feat: add user authentication
# e4f5g6h feat: implement API client
# i7j8k9l fix: resolve routing bug
# m0n1o2p docs: update README
# q3r4s5t refactor: clean up components

echo -e "\n${GREEN}=== Directory Listing ===${NC}"
ls -lah

# Output:
# total 48K
# drwxr-xr-x  8 user user 4.0K Jan 15 10:30 .
# drwxr-xr-x 12 user user 4.0K Jan 15 09:15 ..
# drwxr-xr-x  2 user user 4.0K Jan 15 10:25 dist
# -rw-r--r--  1 user user  156 Jan 15 09:30 .gitignore
# drwxr-xr-x  8 user user 4.0K Jan 15 10:30 node_modules
# -rw-r--r--  1 user user 2.1K Jan 15 10:15 package.json
# -rw-r--r--  1 user user  12K Jan 15 10:15 package-lock.json
# -rw-r--r--  1 user user 1.5K Jan 15 09:45 README.md
# drwxr-xr-x  5 user user 4.0K Jan 15 10:20 src

echo -e "\n${YELLOW}=== Running Tests ===${NC}"
npm test

# Output:
# > subliminal-nightfall@1.0.0 test
# > jest
#
# PASS  src/utils/api.test.ts
#   ✓ should fetch users successfully (15ms)
#   ✓ should handle errors correctly (8ms)
#   ✓ should retry on network failure (25ms)
#
# PASS  src/components/UserList.test.tsx
#   ✓ renders user list correctly (42ms)
#   ✓ handles empty state (18ms)
#   ✓ filters users by search query (31ms)
#
# Test Suites: 2 passed, 2 total
# Tests:       6 passed, 6 total
# Snapshots:   0 total
# Time:        3.456s

echo -e "\n${MAGENTA}=== System Information ===${NC}"
uname -a

# Output:
# Darwin MacBook-Pro.local 23.2.0 Darwin Kernel Version 23.2.0
# x86_64 i386 MacBookPro16,1 Darwin

echo -e "\n${CYAN}=== Process Status ===${NC}"
ps aux | head -n 5

# Output:
# USER   PID  %CPU %MEM      VSZ    RSS   TT  STAT STARTED      TIME COMMAND
# user  1234  15.2  2.1  5432100 345678   ??  S    10:15AM   1:23.45 /Applications/Code.app
# user  5678   8.3  1.5  3210000 234567   ??  S    10:20AM   0:45.12 node server.js
# user  9012   2.1  0.8  1234567 123456   ??  S    10:25AM   0:12.34 npm run dev
# user  3456   0.5  0.3   654321  45678 s001  S    10:30AM   0:03.21 bash

echo -e "\n${GREEN}=== Building Project ===${NC}"
npm run build

# Output:
# > subliminal-nightfall@1.0.0 build
# > astro build
#
# 14:32:15 [content] Types generated 158ms
# 14:32:15 [build] output: "static"
# 14:32:15 [build] directory: /Users/user/project/dist/
# 14:32:15 [build] Collecting build info...
# 14:32:16 [build] ✓ Completed in 1.23s.
# 14:32:16 [build] Building static entrypoints...
# 14:32:18 [build] ✓ Completed in 2.45s.
#
# @astrojs/sitemap: Generated sitemap
#
# ✓ Built in 3.68s

echo -e "\n${RED}=== Error Example ===${NC}"
# This command will fail
cat nonexistent-file.txt 2>&1

# Output:
# cat: nonexistent-file.txt: No such file or directory

echo -e "\n${BLUE}=== Network Request ===${NC}"
curl -s https://api.github.com/users/mhamrah | jq '.name, .bio'

# Output:
# "Michael Hamrah"
# "Software Engineer | Open Source Enthusiast"

echo -e "\n${GREEN}=== Success Message ===${NC}"
echo "✓ All operations completed successfully!"
