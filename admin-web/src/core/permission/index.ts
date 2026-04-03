export function hasPermission(userPerms: string[], requiredPerm: string) {
  if (!requiredPerm) {
    return true;
  }
  if (userPerms.includes("*:*:*")) {
    return true;
  }
  return userPerms.includes(requiredPerm);
}
