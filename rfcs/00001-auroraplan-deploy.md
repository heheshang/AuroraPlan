# Feature

- Feature Name: auroraplan-deploy
- Start Date: 2023-11-02 

## Summary

use github actions to deploy auroraplan & and use cargo nextest to test auroraplan
use docker to deploy auroraplan  & and use cargo nextest to test auroraplan

## Motivation

With this feature, we can deploy auroraplan to github pages and docker, and use cargo nextest to test auroraplan

## Guide-level explanation

Explain the proposal as if it was already included in the language and you were teaching it to another Rust programmer. That generally means:

- Introducing new named concepts.
- Explaining the feature largely in terms of examples.
- Explaining how Rust programmers should *think* about the feature, and how it should impact the way they use Rust. It should explain the impact as concretely as possible.
- If applicable, provide sample error messages, deprecation warnings, or migration guidance.
- If applicable, describe the differences between teaching this to existing Rust programmers and new Rust programmers.

For implementation-oriented RFCs (e.g. for compiler internals), this section should focus on how compiler contributors should think about the change, and give examples of its concrete impact. For policy RFCs, this section should provide an example-driven introduction to the policy, and explain its impact in concrete terms.

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

## Prior art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

## Unresolved questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

## Future possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
# aurora-ui docker deploy

- docker pull nginx
- cd aurora-ui
- npm run build:prod
- cp -r dist/* ~/dist
- docker run --name aurora-ui -d -p 5174:80 -v ~/dist:/usr/share/nginx/html nginx

```shell
server {  
        listen 80;  
        server_name tx;  # 修改为您的域名  


        root /usr/share/nginx/html;  # 修改为您的静态资源路径  
        location ~ ^/ui.*$ {  
            add_header Access-Control-Allow-Credentials false;
            add_header Access-Control-Allow-Origin *;
            add_header Access-Control-Allow-Methods 'GET, POST, OPTIONS';
            add_header Access-Control-Allow-Headers 'DNT,X-Mx-ReqToken,Keep-Alive,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Authorization';
            if ($request_method = 'OPTIONS') { return 204; }
            # proxy_pass http://127.0.0.1:5174/;  # 修改为您的静态资源路径
                try_files $uri $uri/ /index.html;  
                # index index.html index.htm;  
        }  



        location /api {  # 修改为适用于您的后端服务的路径  
            proxy_pass http://myhost:54321;  # 修改为后端服务器的地址  
            proxy_set_header Host $host;  
            proxy_set_header X-Real-IP $remote_addr;  
        }  
}   



```
```shell
 docker run --add-host=myhost:172.18.0.1 -di --name=nginx -p 5174:80 -v /home/ssk/dist:/usr/share/nginx/html  -v /home/ssk/aurora-ui.conf:/etc/nginx/conf.d/aurora-ui.conf nginx
 docker run --add-host=myhost:109.254.3.222 -di --name=nginx -p 5174:80 -v D:\\soft\\workspace\\rustworkspace\\AuroraPlan\\web\aurora-ui\\dist:/usr/share/nginx/html  -v D:\\soft\\workspace\\rustworkspace\\AuroraPlan\\ng-aurora-ui.conf:/etc/nginx/conf.d/aurora-ui.conf nginx
```
```shell
pg_dump -U root -W -F p --clean  --if-exists  --inserts  -f dump.sql dolphinscheduler
```
```bash
AuroraPlan> docker-compose -f  docker-compose-postgres.yml up dolphinscheduler-postgresql -d

PS D:\soft\workspace\rustworkspace\AuroraPlan\crates\lib-models> sqlx database  create
PS D:\soft\workspace\rustworkspace\AuroraPlan\crates\lib-models> sqlx migrate run
Applied 20231206052828/migrate init setup (6.2568ms)
Applied 20231206052930/migrate init (382.2138ms)
Applied 20231206053033/migrate env view crate (5.4882ms)
Applied 20231207014140/migrate table upadte trigger (19.4401ms)
```
