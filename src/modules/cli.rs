//NOTE: file that only contains code about how the cli works [exposes methods to MODULES only]
pub enum CliArgs {
    Command, // for modules, which are global and persist for modules and not it's children
    Basic,   // submodule specific commands/arguments
             //has some error msg txt when the command isnt recognized
}
//Basic commands can be piped to Command, but not vise versa
