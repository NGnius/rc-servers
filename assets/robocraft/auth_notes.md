# Login

Seems to use `Login.RoboAuthService`.

These are mostly assumptions and untested.

## Standard login

Username/password login calls `AuthWithIdEnumerator(string identifier, string password, Dictionary<string, object> dataToReturn)`, which instantiates the class `#=zTk$8WYeWP0gxjV8edvqGncg=` (nested class in `RoboAuthService`). It sets the following variables:

```
#=zTk$8WYeWP0gxjV8edvqGncg=.#=zOuYEgQM= = identifier;
#=zTk$8WYeWP0gxjV8edvqGncg=.#=zBLK$yEM= = password;
#=zTk$8WYeWP0gxjV8edvqGncg=.#=zmTDf_G1vkOnQ = dataToReturn;
```

`identifier` is presumably user display name. No idea what `dataToReturn` contains.

## Steam Login

The first time, steam login calls `RegisterAndAuthenticateSteam(string validDisplayName, string ticket, Action<Dictionary<string, object>> onAuthSuccess, Action<Exception> onError)`, which calls `#=zeCm7CtCIMhaXGczndJSgBvo=(...)`, which instantiates `#=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==` (nested class in `RoboAuthService`) and schedules it in the TaskRunner (it's an enumerator).

```
// in RegisterAndAuthenticateSteam
RoboAuthService.#=zeCm7CtCIMhaXGczndJSgBvo=(validDisplayName, ticket, onAuthSuccess, onError);

// which calls
[DebuggerHidden]
private static IEnumerator #=zeCm7CtCIMhaXGczndJSgBvo=(string #=zgoyh9IIuSHQ$, string #=zI$MKIcRAkgM$, Action<Dictionary<string, object>> #=zmoN68pYsK05$, Action<Exception> #=zN4ekGCs=)
{
    #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg== #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg== = new #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==();
    #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==.#=zI$MKIcRAkgM$ = #=zI$MKIcRAkgM$;
    #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==.#=zgoyh9IIuSHQ$ = #=zgoyh9IIuSHQ$;
    #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==.#=zmoN68pYsK05$ = #=zmoN68pYsK05$;
    #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==.#=zN4ekGCs= = #=zN4ekGCs=;
    return #=zudtivLXmdm5aL7OqfX$JZA2ozi4nV_Rakg==;
}
```

If it's not the first time, steam login calls `AuthenticateSteamUser(Action<Dictionary<string, object>> onSuccess, Action onFailure, Action<Exception> onError)`, which calls `AuthenticateSteamUserInternal(...)` and then schedules the task (IEnumerator is returned).

```
[DebuggerHidden]
public static IEnumerator AuthenticateSteamUserInternal(Action<Dictionary<string, object>> onSuccess, Action onFailure, Action<Exception> onError)
{
    #=zxQRony8oFWvBWu4O0RraT0fsXcjv #=zxQRony8oFWvBWu4O0RraT0fsXcjv = new #=zxQRony8oFWvBWu4O0RraT0fsXcjv();
    #=zxQRony8oFWvBWu4O0RraT0fsXcjv.#=zmoN68pYsK05$ = onSuccess;
    #=zxQRony8oFWvBWu4O0RraT0fsXcjv.#=zN4ekGCs= = onError;
    return #=zxQRony8oFWvBWu4O0RraT0fsXcjv;
}
```
