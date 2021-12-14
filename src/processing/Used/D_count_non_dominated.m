addpath(".");

close all
clear
clc
format long g

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "data", "Comparison");
out_folder = fullfile(root_path, "processed", "nd");

if ~exist(out_folder, 'dir')
    mkdir(out_folder)
end

sizes = ["12", "16", "20", "24", "28", "32"];

cmp_alg = fullfile("qm", "MOEAD");

other_algs = [
    fullfile("esp_vdce", "*"), fullfile("bfdsu", "*"),...
    fullfile("binary", "MOEAD"), fullfile("std", "MOEAD"),...
    fullfile("stringer", "*")];
out_algs = ["esp_vdce", "bfdsu", "binary", "std", "stringer"];

num_runs = 30;

for i = 1:length(other_algs)
    other_alg = other_algs(i);
    out_alg = out_algs(i);
    
    rows = [];
    
    for j = 1:length(sizes)
        size = sizes(j);
        
        cnt_diff = zeros(num_runs, 1);
        for k = 0:num_runs-1
            run = num2str(k);
            
            cmp_objs = get_objs(src_folder, size, run, cmp_alg);
            
            other_objs = get_objs(src_folder, size, run, other_alg);
            other_objs = unique(other_objs, 'rows');
            original_cnt = length(other_objs);
            
            after_objs = filter_NDS(other_objs, cmp_objs);
            post_cnt = length(after_objs);
            
            diff = original_cnt - post_cnt;
            
            cnt_diff(k+1) = (diff / original_cnt) * 100;
        end
        
        md = median(cnt_diff);
        lq = prctile(cnt_diff, 25, 1);
        uq = prctile(cnt_diff, 75, 1);
        
        lq_ = md - lq;
        uq_ = uq - md;
        
        out_size = (str2double(size) ^ 3) / 4;
        rows(j, :) = [out_size; lq_; md; uq_];
    end
    
    out_file = fullfile(out_folder, append(out_alg, '.csv'));
    fid = fopen(out_file, 'w');
    
    csvwrite(out_file, rows);
end

fclose(fid);

function objectives = get_objs(root_folder, size, run, alg_name)

file_search = fullfile(root_folder, size, run, alg_name, '*.objs');
item = dir(file_search);

folder = item.folder;
file_name = item.name;

num_services_idx = strfind(file_name, '_');
num_services = extractBetween(file_name, 1, num_services_idx - 1);
num_services = str2double(num_services{1});

full_path = fullfile(folder, file_name);

% Manually read CSV to handle 'Infeasible' values
fid = fopen(full_path);
lines = {};
tline = fgetl(fid);

objectives = [];
row = 1;

while ischar(tline)
    if contains(tline, 'Infeasible')
        tline = fgetl(fid);
        continue
    end
    
    s = split(tline, ',');
    
    objectives(row,1) = str2num(s{1});
    objectives(row,2) = str2num(s{2});
    objectives(row,3) = str2num(s{3});
    
    tline = fgetl(fid);
    
    row = row + 1;
end
fclose(fid);

if ~isempty(objectives)
    objectives(:, 3) = objectives(:, 3) / num_services;
end

end