# Highlighting matching brackets in Zed
> 🕰️ logged on 2025.03.26 <br>
> 🔖 tagged: development, ide, editors, code-highlighting

I'm currently trying out Zed as an editor for my personal projects. Although I am very impressed, Zed does have its shortcomings...

In its current state Zed does not support rainbow brackets. I find this to be quite cumbersome as I'm not able to glance if there are missing brackets in a large chunk of code; this is not optimal for my productivity to say the least.

Fortunately [OddMastodon4038 on Reddit](https://old.reddit.com/r/ZedEditor/comments/1er81tt/is_bracket_highlighting_a_feature/lhymm51/) had a nice workaround that I actually ended up liking way more than using full-on rainbow brackets. Zed is quite minimal and after enabling only the bare minimal productivity ~~distractions~~ features I actually noticed being able to focus better! Neat!

> [!Tip]
> You can find my full Zed configuration file in [my dotfiles repository](https://github.com/bvdbasch/dotfiles/tree/master/roles/zed)!

Anway, here are the highlighting settings I've used:
```json
"indent_guides": {
  "background_coloring": "disabled",
  "coloring": "indent_aware",
  "active_line_width": 1
},
"experimental.theme_overrides": {
  "editor.document_highlight.read_background": "#6A3D88"
}
```

## Sources
+ [Is bracket highlighting a feature?](https://old.reddit.com/r/ZedEditor/comments/1er81tt/is_bracket_highlighting_a_feature/lhymm51/)
