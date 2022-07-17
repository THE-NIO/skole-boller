# NX: THe how and the what

NX is an orchestration and mono-repo tool designed for the purpose of managing large projects and suits of apps in a simple and elegant manner. NX has four main components:

1. Workspace
   - A collection of libs and apps that are collected in one repo that are easily run- and fetchable.
2. Project

- This is either an app or a library.
- If its a library its accessible with the npm scope @the-nio. The index of libraries is automatically updated. If you want to see the index take a look in `tsconfig.base.json`

3. Generators
   - Scaffold and boilerplate generators designed to make the life of the developer easier
   - These generators can either be run using the NX VSC-EXT or using the nx cli. First ensure you have the NX cli available by running `nx`. If its not available install it globally using `npm i -g nx`.
   - The most common generators we will use:
     - Frontend
       - `nx generate @nxext/svelte:lib --e2eTestRunner none --name PROJECT-NAME && nx generate @nxext/svelte:storybook-configuration --name PROJECT-NAME`
       - `nx generate @nxext/svelte:app --e2eTestRunner cypress --name PROJECT-NAME`
       - `nx generate @nxext/component --name COMPONENT-NAME --project PROJECT-NAME`
     - Backend
       - `nx generate @nxrs/cargo:bin --name PROJECT-NAME`
       - `nx generate @nxrs/cargo:lib --name PROJECT-NAME`
4. Executors
  - Executors is where NX's orchestration capabilities come in. They are how you run a command for a project; any number of project or git-affected projects
  - Most developers will only run one app at a time. For this use `nx run PROJECT-NAME:EXECUTOR-NAME`
  - You are probably more used to more traditional commands. Their NX equivalents are
    - `cargo build` &rarr; `nx run PROJECT-NAME:build`
    - `cargo check` &rarr; `nx run PROJECT-NAME:lint`
    - `npm run serve` &rarr; `nx run PROJECT-NAME:serve`
