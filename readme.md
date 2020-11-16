## kyoku
rusty music server

meant to be used with [waveline player](https://player.waveline.app) but currently not compatible.

note: this is absolutely not done by any means. anything can and will change.
### API
#### System
|Done|Path|Description|Queries/Notes
|-|-|-|-|
[ ]|`GET /system/info`|Get stats about the server|N/A|

#### Tracks
|Done|Path|Description|Queries/Notes
|-|-|-|-|
[ ]|`GET /tracks/:id`|Get information about a track|N/A|
[ ]|`GET /tracks/:id/stream`|Stream a track|`?transcode=:bool` - transcode the song or not|
||||`?rename=:bool` - rename the song upon downloading|
[ ]|`GET /tracks/:id/like`|Get whether you liked the track|`?liked=:bool` - toggle song likage|

#### Albums
|Done|Path|Description|Queries/Notes
|-|-|-|-|
[ ]|`GET /albums/`|Get all albums|`?skip=:int`/`?index=:int` - which index to start at (used for paging)|
||||`?limit=:int` - the max albums the endpoint will return, defaults to 10
||||`?artist=:string` - self-explanatory
[ ]|`GET /albums/:id`|Get information about an album|N/A|
[ ]|`GET /albums/:id/art`|Get an album's art|N/A|

#### Artist
|Done|Path|Description|Queries/Notes
|-|-|-|-|
[ ]|`GET /artists/`|Get all artists|`?skip=:int`/`?index=:int` - which index to start at (used for paging)|
||||`?limit=:int` - the max artists the endpoint will return, defaults to 10
[ ]|`GET /artists/:id`|Get information about an artist|N/A|

#### Search
|Done|Path|Description|Queries/Notes
|-|-|-|-|
[ ]|`GET /search`|Get information about a track|`?q=:string` - search for your query!|
||||Note: returns all categories