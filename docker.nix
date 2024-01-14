{
  dockerTools,
  demostf-backup,
}:
dockerTools.buildImage {
  name = "demostf/backup";
  tag = "latest";
  copyToRoot = [demostf-backup];
  config = {
    Cmd = ["${demostf-backup}/bin/demostf-backup"];
  };
}
