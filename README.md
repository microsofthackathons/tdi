_⚠ This is a hackathon project with no official support or quality guarantee_

Hackathon 2022

# tdi

The command-line interface, for some, is the natural way to work on a computer.  The "To Do" app is the standard, simple interface to a simple problem.  Microsoft has a wonderful desktop/GUI application for doing such things - https://todo.microsoft.com/tasks/.  **tdi** (short for To Do Interface) intends to create a CLI app and interface to Microsoft Graph, the store for Microsoft To Do's managed tasks.  The app will be built in and released through Github - hoping for Windows, macOS and Linux builds.

## License

MIT

## Development

There is an "official" application registration in Azure for the client.  But if you're wanting to use your own for development you can follow these steps:

https://learn.microsoft.com/en-us/azure/healthcare-apis/register-application

You will need both the `CLIENT_ID` and `CLIENT_SECRET`.  In addition you'll need to add the appropriate API permissions for `user.read` and `tasks.readwrite` in order for the application to access the Graph API.  With the credentials in hand you will then add them into the source code before building - here: https://github.com/microsofthackathons/tdi/blob/develop/src/tasks.rs#L17

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft 
trademarks or logos is subject to and must follow 
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.

## Bill of Materials

A SBOM is generated, and captured in the repo at `bom.xml` in CycloneDX format.

To produce the SBOM, from the project repo's root directory run:

```
$ cargo install cargo-cyclonedx
$ cargo cyclonedx
```