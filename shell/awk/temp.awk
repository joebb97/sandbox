#!/usr/bin/awk -f
BEGIN {}
{
    if (substr($1, length($1), 1)  == ":") {
        print tolower($1)
        $1=""
        print $0
    } else  {
    print $0
}
}
