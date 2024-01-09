## Overview of TODO's / Plan

- Installation wizard that allows for out of the box build and running of the project
- Configuration system to allow the user to select specific crates to use with their project. E.G. actix-server vs warp server, or postgresql database vs seaorm database. SQL / ORM
- Dynamic API that builds the routing system based on the selected configuration.
- Protobuf so that the routing is set as a configuration and then applied to whichever server system that the user has selected.
- Database tables that are easily extendable.
- Built in authentication and authorization, login system with security on the routes based on permissions. Default administration account with full access to the site and it's functionality
- SSH Logins / Registration, Regular Username / Password, OAuth Modern integration with 3rd party systems.
- UI theming system that allows the user to dynamically change the layout, colourscheme typography and other attributes of the user interface. This will have a core structure for each component such as a login form must require X to meet Y function, eg, username and password fields are required for a login component if the type of login authentication is set to username/password, etc...
- Allow for modular code and to have each individual aspect of the site to be interchangeable with other more prefereable solutions if so desired. For example the authentication logic could be switched out easily and once the function has been changed then it seemlessly works as middleware with the selected server solution defined in the config.
