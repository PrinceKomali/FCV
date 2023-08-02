import std.stdio, std.process;
extern (C) void start_bot();

void main(string[] args) {
    if(args.length > 1) {
        switch(args[1]) {
            case "init":
                // init_db();
                break;
            default:
                writeln("Invalid Argument");
                break;
        }
        return;
    }
    start_bot();
}