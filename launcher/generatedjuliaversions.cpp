module JuliaVersionDatabase;

std::vector<JuliaVersion> JuliaVersionsDatabase::getHardcodedJuliaVersions() {
	std::vector<JuliaVersion> juliaVersions = { 
    JuliaVersion{0, 7, 0}, JuliaVersion{1, 0, 0}, JuliaVersion{1, 0, 1}, JuliaVersion{1, 0, 2}, JuliaVersion{1, 0, 3}, JuliaVersion{1, 0, 4}, JuliaVersion{1, 0, 5}, JuliaVersion{1, 1, 0}, JuliaVersion{1, 1, 1}, JuliaVersion{1, 2, 0}, JuliaVersion{1, 3, 0}, JuliaVersion{1, 3, 1}, JuliaVersion{1, 4, 0}, JuliaVersion{1, 4, 1}, JuliaVersion{1, 4, 2}, JuliaVersion{1, 5, 0}, JuliaVersion{1, 5, 1}, JuliaVersion{1, 5, 2}, JuliaVersion{1, 5, 3}, JuliaVersion{1, 5, 4}, JuliaVersion{1, 6, 0}, JuliaVersion{1, 6, 1}
	};
  
	return juliaVersions;
}

std::wstring JuliaVersionsDatabase::getBundledJuliaVersion() {
  return std::wstring {L"1.6.1"};
}
