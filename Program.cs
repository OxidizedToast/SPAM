using System;
using System.IO;

namespace spam {
  class Program {
    static void Main(String[] args)
    {
      // checks is system is supported
      if(!OperatingSystem.IsLinux()){
          Console.WriteLine("Sorry, only linux is supported currently.");
          Environment.Exit(0);
      }
      Version version_number = new Version(0, 0, 2);
      const string Title = @"  
  █████████  ███████████    █████████   ██████   ██████
 ███░░░░░███░░███░░░░░███  ███░░░░░███ ░░██████ ██████ 
░███    ░░░  ░███    ░███ ░███    ░███  ░███░█████░███ 
░░█████████  ░██████████  ░███████████  ░███░░███ ░███ 
 ░░░░░░░░███ ░███░░░░░░   ░███░░░░░███  ░███ ░░░  ░███ 
 ███    ░███ ░███         ░███    ░███  ░███      ░███ 
░░█████████  █████        █████   █████ █████     █████
 ░░░░░░░░░  ░░░░░        ░░░░░   ░░░░░ ░░░░░     ░░░░░";

      Console.Clear();
      Console.WriteLine(Title);
      Console.WriteLine("Version: " + version_number);
      Console.WriteLine("\n\nWhat would you like to do?: ");
      Console.WriteLine(@"
          [1] Toggle (toggle)
          [2] Status (status)
          [3] Help (help)
          [4] Exit (exit)");
      string? choice = Console.ReadLine()?.Trim().ToLower();
      if(choice == "1" || choice == "toggle") {
          Console.WriteLine("toggling Daemon...");
      } else if(choice == "2" || choice == "status") {
          Status.CheckStatus();
      } else if(choice == "3" || choice == "help"){
          Console.WriteLine(@"Options: 
          [1] Toggle (toggle) # Toggles the status of daemon
          [2] Status (status) # Returns status / state of daemon(on/off)
          [3] Help (help)     # Displays this screen
          [4] Exit (exit)     # Exits program
          Coded by: OxidizedToast
              ");
      } else if(choice == "4" || choice == "exit") {
              Console.WriteLine("Goodbye.");
              Environment.Exit(0);
      } else {
          Console.WriteLine("Option not recognized.");
      }
        
    }
  }
}
