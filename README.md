### MedPass 

This is the backend service implementation for a mobile app
destined to handle personal medical files. 


#### Proto files
The service loads the proto fies using protofetch 

* First install protofetch with `cargo install protofetch`
* Get proto files `protofetch fetch`
* Update protofiles `protofetch update` and then `protofetch fetch`

Tonic configuration for rust code generations is found in `build.rs`.

Before running the service, execute `scripts/init_db.sh`. Spins up a posgtres and runs 
sqlx migrations. 

### Environment variables
Set the following environment variables before running the service:

<pre>
export APP__DB_CONFIG__PASSWORD="postgres"
export APP__STORAGE_CONFIG__ACCESS_KEY_ID="..." // scaleway s3 object storage access key
export APP__STORAGE_CONFIG__SECRET_ACCESS_KEY="..." //scaleway s3 access key
</pre>



