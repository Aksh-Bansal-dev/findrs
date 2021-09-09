pub fn print_help(){
    print!(
        "\n\
        findrs: search for patterns in files\n\n\
        Usage: findr <PATTERN> <FILE_PATH> [OPTIONS]\n\n\
        Options: \n\
        \t{: <20}\t {: <40}\n\
        \t{: <20}\t {: <40}\n\
        \t{: <20}\t {: <40}\n\
        ", 
        "-i, --ignore-case" ,
        "Case insensitive search",
        "-h, --help",
        "display help for findrs",
        "-d, --dir",
        "search all the files in the directory"
    );
}
