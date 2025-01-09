# Devfile Handling

Any devfile related stuff will be handled here.

## Restart from needed

Well has we are in the process in setting up (rewriting) the restart from [local|url] feature we need to understand what they made and how it works

### Useful link

- <https://github.com/che-incubator/che-code/blob/main/code/extensions/che-remote/src/extension.ts#L75> The code is where the command is setup in che-code.
- <https://github.com/che-incubator/che-code/blob/main/code/extensions/che-api/src/impl/k8s-devfile-service-impl.ts#L73> This is the place where the devfile is updated
- <https://github.com/devfile/devworkspace-generator/blob/main/src/main.ts#L32> this code handle the generation of the first step of generating your new devfile
- <https://github.com/devfile/devworkspace-generator/blob/main/src/generate.ts#L37> And here another midlle step that will call the context generator the contente will be generated line 65
- <https://github.com/devfile/devworkspace-generator/blob/main/src/devfile-schema/devfile-schema-validator.ts> This module handle the DevFile Validator (split into devfile version thing that i will need to do)

This part should be done in JS and based on the library made by the che team. BUT i want to do a full rust version of it. So the lib will be translated in rust.

### What we need

- [x] A [validator](https://docs.rs/jsonschema/latest/jsonschema/) that will make sure that our devfile follow the intended format. (The lib need to be tested)
- [ ] A fetch that will handle recuperating the content of the devfile (existing one(the one in the devworkspaces or the local one) ++ a possible remote one (link) ++ check if the folder `.che` include a file `che-editor.yaml`)
- [ ] An aggregator that will handle including the editor/merging component/ etc
- [ ] An updater that will update the devfile inside the devspaces (that is present in the CRD)
- [ ] A lifecycle handler ?
  - [ ] [Patch](https://docs.rs/kube/latest/kube/api/enum.Patch.html)
  - [ ] [Stop WS](https://github.com/che-incubator/che-code/blob/6e0a908d58cacb380c216dde3af544d75e3913d5/code/extensions/che-api/src/impl/k8s-workspace-service-impl.ts#L62)


### Useful V2

- [Git Resolver](https://github.com/devfile/devworkspace-generator/tree/main/src/resolve)
- 