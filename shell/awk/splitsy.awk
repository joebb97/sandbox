function getLastOfSplit(string, splitOn) {
    split(string, splitResult, splitOn)
    idx = length(splitResult)
    return splitResult[idx]
}

BEGIN { unknown = 0 }
{
    checkingCPE = NR % 2
    if (checkingCPE) {
        if (!match($0, /cpe[: ]/)) {
            printf("Bad format of cpe comment on line %d: %s", NR, $0)
            exit
        }
        # Allow for comments of "# cpe unknown" which will have NF == 3
        if (NF != 2) {
            unknown=1
            next
        }
        expectedVer = getLastOfSplit($2, ":")
    } else if (!unknown) {
        # Otherwise we're checking a package listing
        if (NF != 1 || index($0, "-") == 0) {
            printf("Bad format of package on line %d: %s", NR, $0)
            exit
        }
        actualVer = getLastOfSplit($1, "-")
        if (expectedVer != actualVer) {
            printf("Mismatch for package %s: got %s and %s", $1, 
                   expectedVer, actualVer)
        }
    }
    unknown = 0
}
