#!/bin/bash

# Phase 2A: Fix Compilation - Automated Start Script
# This script automates the initial setup and first few tasks

set -e  # Exit on error

echo "=================================="
echo "Phase 2A: Fix Compilation"
echo "=================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Change to core directory
cd "$(dirname "$0")/../core" || exit 1

echo -e "${YELLOW}Step 1: Backing up Cargo.toml${NC}"
cp Cargo.toml Cargo.toml.backup
echo -e "${GREEN}✓ Backup created${NC}"
echo ""

echo -e "${YELLOW}Step 2: Adding missing dependencies${NC}"

# Add futures
if ! grep -q "^futures = " Cargo.toml; then
    echo 'futures = "0.3"' >> Cargo.toml
    echo -e "${GREEN}✓ Added futures${NC}"
fi

# Add governor for rate limiting
if ! grep -q "^governor = " Cargo.toml; then
    echo 'governor = "0.6"' >> Cargo.toml
    echo -e "${GREEN}✓ Added governor${NC}"
fi

# Add memmap2
if ! grep -q "^memmap2 = " Cargo.toml; then
    echo 'memmap2 = "0.9"' >> Cargo.toml
    echo -e "${GREEN}✓ Added memmap2${NC}"
fi

echo ""
echo -e "${YELLOW}Step 3: Updating existing dependencies${NC}"

# Update axum to include ws feature
sed -i 's/axum = "0.7"/axum = { version = "0.7", features = ["ws"] }/' Cargo.toml
echo -e "${GREEN}✓ Updated axum with ws feature${NC}"

# Update reqwest to include blocking feature
sed -i 's/reqwest = { version = "0.11", features = \["json"\] }/reqwest = { version = "0.11", features = ["json", "blocking"] }/' Cargo.toml
echo -e "${GREEN}✓ Updated reqwest with blocking feature${NC}"

echo ""
echo -e "${YELLOW}Step 4: Running initial cargo check${NC}"
if cargo check 2>&1 | tee /tmp/cargo_check_initial.txt; then
    echo -e "${GREEN}✓ Initial check passed${NC}"
else
    ERROR_COUNT=$(grep -c "error:" /tmp/cargo_check_initial.txt || echo "0")
    echo -e "${RED}✗ Found $ERROR_COUNT errors${NC}"
    echo "See /tmp/cargo_check_initial.txt for details"
fi

echo ""
echo -e "${YELLOW}Step 5: Creating progress tracker${NC}"
cat > ../phase2a_progress.txt << 'EOF'
[x] Task 1: Add Missing Dependencies (AUTOMATED)
[ ] Task 2: Fix Duplicate Functions
[ ] Task 3: Add Trait Implementations
[ ] Task 4: Fix Method Signatures
[ ] Task 5: Fix Type Mismatches
[ ] Task 6: Fix Missing Methods
[ ] Task 7: Fix Missing Modules
[ ] Task 8: Fix Argument Mismatches
[ ] Task 9: Fix Missing Fields
[ ] Task 10: Fix System Module
[ ] Task 11: Fix Remaining Issues
[ ] Task 12: Final Verification
EOF
echo -e "${GREEN}✓ Progress tracker created${NC}"

echo ""
echo "=================================="
echo -e "${GREEN}Phase 2A Setup Complete!${NC}"
echo "=================================="
echo ""
echo "Next steps:"
echo "1. Review PHASE2A_ACTION_PLAN.md"
echo "2. Run: ./scripts/fix_duplicates.sh"
echo "3. Run: ./scripts/add_traits.sh"
echo "4. Continue with manual fixes as needed"
echo ""
echo "Progress tracker: phase2a_progress.txt"
echo "Backup: core/Cargo.toml.backup"