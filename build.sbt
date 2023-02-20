ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "2.13.10"

lazy val root = (project in file("."))
  .settings(
    name := "lancelab",
    libraryDependencies ++= Seq(
      "org.apache.arrow" % "arrow-c-data"       % "11.0.0",
      "org.apache.arrow" % "arrow-vector"       % "11.0.0",
      "org.apache.arrow" % "arrow-memory-core"  % "11.0.0",
      "org.apache.arrow" % "arrow-memory-netty" % "11.0.0",
      "org.apache.arrow" % "arrow-format"       % "11.0.0"
    )
  )
