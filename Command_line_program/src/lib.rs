//An I/O Project: Building a Command Line Program

//for our project, we’ll make our own version of the classic command line tool grep (globally search a regular expression and print). 
//In the simplest use case, grep searches a specified file for a specified string. 
//To do so, grep takes as its arguments a filename and a string.

//Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

//We’ll read the value of an environment variable to allow the user to configure the behavior of our tool.
//We’ll also print error messages to the standard error console stream (stderr) instead of standard output (stdout), 
//so, for example, the user can redirect successful output to a file while still seeing error messages onscreen.



//ACCEPTING COMMAND LINE ARGUMENTS
//Let’s create a new project with, as always, cargo new. We’ll call our project minigrep to distinguish it from the grep tool that you might already have on your system.
//check minigrep