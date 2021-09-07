pub fn print_help(){
    print!(
        "\n\
        findrs: search for pattern in file\n\n\
        Usage: findr <PATTERN> <FILE_PATH> [OPTIONS]\n\n\
        Options: \n\
        \t{: <20}\t {: <40}\n\
        \t{: <20}\t {: <40}\n\
        ", 
        "-i, --ignore-case" ,
        "Case insensitive search",
        "-h, --help",
        "display help for findrs"
    );
}
