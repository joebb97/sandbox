BEGIN {}
{
    if ($0 ~ /\[nginx\]/ && $0 ~ /host="wiki.aledade.com"/) {
        # print
        split($7,a,"=")
        part = a[2]
        if (part != "-") {
            len = length(part)
            if (substr(part, len, 1) == ",") {
                part = substr(part, 1, len - 1)
            }
            if (int(part) > 50) {
                print
            }
        }
    }
}
END {}
