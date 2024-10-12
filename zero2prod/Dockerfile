 #WeusethelatestRuststablereleaseasbaseimage
 FROM rust:1.59.0
 #Let's switchourworkingdirectoryto`app`(equivalentto`cdapp`)
 #The`app` folderwillbecreatedforusbyDockerincaseitdoesnot
 #exist already.
 WORKDIR /app
 #Installtherequiredsystemdependenciesforourlinkingconfiguration
 RUN aptupdate&&aptinstalllldclang-y
 #Copy allfilesfromourworkingenvironmenttoourDockerimage
 COPY . .
 #Let's buildourbinary!
 #We'll use thereleaseprofiletomakeitfaaaast
 RUN cargobuild--release
 #When `dockerrun`isexecuted,launchthebinary!
 ENTRYPOINT ["./target/release/zero2prod"]